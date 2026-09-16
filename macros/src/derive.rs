use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Data, DeriveInput, Fields, Ident, Lifetime, LitChar, LitStr, Token, bracketed,
    parse_macro_input,
};

const MAX_FIELDS: usize = 32;

pub fn derive_pickable(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match expand(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    match &input.data {
        Data::Struct(_) => expand_struct(input),
        Data::Enum(_) => expand_enum(input),
        Data::Union(_) => Err(syn::Error::new_spanned(
            &input.ident,
            "derive can only be applied to structs or enums",
        )),
    }
}

#[derive(Default)]
struct FieldArgOptions {
    short: Option<ShortArg>,
    long: Option<LongArg>,
    aliases: Vec<String>,
}

enum ShortArg {
    Implicit,
    Explicit(char),
}

enum LongArg {
    Implicit,
    Explicit(String),
}

enum FieldArgItem {
    Short(ShortArg),
    Long(LongArg),
    Aliases(Vec<String>),
}

impl Parse for FieldArgItem {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        if key == "short" {
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                let lit: LitChar = input.parse()?;
                Ok(Self::Short(ShortArg::Explicit(lit.value())))
            } else {
                Ok(Self::Short(ShortArg::Implicit))
            }
        } else if key == "long" {
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                let lit: LitStr = input.parse()?;
                Ok(Self::Long(LongArg::Explicit(lit.value())))
            } else {
                Ok(Self::Long(LongArg::Implicit))
            }
        } else if key == "aliases" {
            input.parse::<Token![=]>()?;
            let content;
            bracketed!(content in input);
            let aliases = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;
            Ok(Self::Aliases(
                aliases.into_iter().map(|s| s.value()).collect(),
            ))
        } else {
            Err(syn::Error::new(
                key.span(),
                "expected `short`, `long`, or `aliases`",
            ))
        }
    }
}

fn parse_arg_attr(attr: &Attribute) -> syn::Result<FieldArgOptions> {
    let mut options = FieldArgOptions::default();
    let items = attr.parse_args_with(Punctuated::<FieldArgItem, Token![,]>::parse_terminated)?;
    for item in items {
        match item {
            FieldArgItem::Short(short) => options.short = Some(short),
            FieldArgItem::Long(long) => options.long = Some(long),
            FieldArgItem::Aliases(aliases) => options.aliases = aliases,
        }
    }
    Ok(options)
}

fn derived_short(ident: &Ident) -> char {
    ident.to_string().chars().next().unwrap_or('_')
}

fn build_picker_arg(ident: &Ident, ty: &syn::Type, options: &FieldArgOptions) -> TokenStream2 {
    let root = crate::picker_root();
    let has_options =
        options.short.is_some() || options.long.is_some() || !options.aliases.is_empty();
    if !has_options {
        // Without any `#[arg(...)]` customization, the field is positional:
        // it expands to the same form as `arg![Type]`.
        return quote! {
            #root::PickerArg::<#ty> {
                full: &[],
                short: ::std::option::Option::None,
                positional: true,
                internal_type: ::std::marker::PhantomData,
            }
        };
    }

    let mut full_names = Vec::new();
    if let Some(LongArg::Explicit(long)) = &options.long {
        full_names.push(long.clone());
    } else {
        // `long` is either implicit or absent, so the field name is the
        // primary long name.
        full_names.push(ident.to_string());
    }
    full_names.extend(options.aliases.iter().cloned());

    let full_lits: Vec<_> = full_names.iter().map(|name| quote!(#name)).collect();
    let short_expr = match &options.short {
        Some(ShortArg::Explicit(short)) => quote!(::std::option::Option::Some(#short)),
        Some(ShortArg::Implicit) => {
            let short = derived_short(ident);
            quote!(::std::option::Option::Some(#short))
        }
        None => quote!(::std::option::Option::None),
    };

    quote! {
        #root::PickerArg::<#ty> {
            full: &[#(#full_lits),*],
            short: #short_expr,
            positional: false,
            internal_type: ::std::marker::PhantomData,
        }
    }
}

fn expand_struct(input: DeriveInput) -> syn::Result<TokenStream2> {
    let root = crate::picker_root();
    let struct_data = match &input.data {
        Data::Struct(data) => data,
        _ => unreachable!(),
    };

    let fields = match &struct_data.fields {
        Fields::Named(named) => &named.named,
        Fields::Unnamed(_) | Fields::Unit => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "derive structs must use named fields",
            ));
        }
    };

    if fields.len() > MAX_FIELDS {
        return Err(syn::Error::new_spanned(
            &input.ident,
            format!("derive supports at most {MAX_FIELDS} fields"),
        ));
    }

    if fields.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "derive structs require at least one field",
        ));
    }

    let struct_name = &input.ident;
    let mut field_infos = Vec::with_capacity(fields.len());
    for field in fields {
        let ident = field
            .ident
            .clone()
            .expect("named fields always have an identifier");
        let ty = field.ty.clone();
        let options = field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("arg"))
            .map(parse_arg_attr)
            .transpose()?
            .unwrap_or_default();
        field_infos.push((ident, ty, options));
    }

    let binding_names: Vec<_> = (0..field_infos.len())
        .map(|i| format_ident!("__derive_{i}"))
        .collect();
    let field_idents: Vec<_> = field_infos.iter().map(|(ident, _, _)| ident).collect();

    let arg_exprs: Vec<TokenStream2> = field_infos
        .iter()
        .map(|(ident, ty, options)| build_picker_arg(ident, ty, options))
        .collect();

    let first_arg = &arg_exprs[0];
    let rest_args = &arg_exprs[1..];

    let pick_chain = quote! {
        #root::IntoPicker::pick(raw_strs, &(#first_arg))
        #( .pick(&(#rest_args)) )*
    };

    // Build a precise tag phase: each field claims only what its own
    // `Pickable::tag` returns, using an updated mask just like parse.rs does.
    let tag_blocks: Vec<TokenStream2> = field_infos
        .iter()
        .enumerate()
        .map(|(i, (_, ty, _))| {
            let arg_expr = &arg_exprs[i];
            let arg_var = format_ident!("__derive_field_arg_{i}");
            let info_var = format_ident!("__derive_field_info_{i}");
            let ctx_var = format_ident!("__derive_inner_ctx_{i}");
            let tagged_var = format_ident!("__derive_tagged_{i}");
            quote! {
                {
                    let #arg_var = #arg_expr;
                    let #info_var = #root::PickerArgInfo::from(&#arg_var);
                    let #ctx_var = #root::TagPhaseContext {
                        arg_info: &#info_var,
                        args: ctx.args,
                        mask: &__derive_mask,
                    };
                    let #tagged_var = <#ty as #root::Pickable>::tag(#ctx_var);
                    for &__derive_idx in &#tagged_var {
                        __derive_mask[__derive_idx] = 1;
                    }
                    __derive_tagged.extend(#tagged_var);
                }
            }
        })
        .collect();

    let option_names: Vec<_> = (0..field_infos.len())
        .map(|i| format_ident!("__derive_opt_{i}"))
        .collect();

    let destructure = if field_infos.len() == 1 {
        let option_name = &option_names[0];
        let binding = &binding_names[0];
        quote! {
            let #option_name = #pick_chain.unpack();
            let Some(#binding) = #option_name else {
                return #root::PickerArgResult::NotFound;
            };
        }
    } else {
        let some_bindings: Vec<_> = binding_names
            .iter()
            .map(|binding| quote!(Some(#binding)))
            .collect();
        quote! {
            let (#(#option_names),*) = #pick_chain.unpack();
            let (#(#some_bindings),*) = (#(#option_names),*) else {
                return #root::PickerArgResult::NotFound;
            };
        }
    };

    let original_generics = &input.generics;
    let ty_generics = original_generics.split_for_impl().1;
    let pick_lifetime = Lifetime::new("'__derive", proc_macro2::Span::call_site());
    let original_params = &original_generics.params;
    let impl_generics = if original_params.is_empty() {
        quote! { <#pick_lifetime> }
    } else {
        quote! { <#pick_lifetime, #original_params> }
    };

    let predicates: Vec<TokenStream2> = original_generics
        .where_clause
        .as_ref()
        .map(|wc| wc.predicates.iter().map(|pred| quote!(#pred)).collect())
        .unwrap_or_default();
    let where_clause = if predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#predicates),* }
    };

    let impl_tokens = quote! {
        impl #impl_generics #root::Pickable<#pick_lifetime> for #struct_name #ty_generics #where_clause {
            fn get_attr(
                _flag: &#pick_lifetime #root::PickerArg<#pick_lifetime, Self>,
            ) -> #root::PickerArgAttr {
                #root::PickerArgAttr::Preprocess
            }

            fn tag(ctx: #root::TagPhaseContext) -> ::std::vec::Vec<usize> {
                let mut __derive_mask: ::std::vec::Vec<u8> = ctx.mask.to_vec();
                let mut __derive_tagged: ::std::vec::Vec<usize> = ::std::vec::Vec::new();

                #(#tag_blocks)*

                __derive_tagged
            }

            fn pick(raw_strs: &[&str]) -> #root::PickerArgResult<Self> {
                #destructure

                #root::PickerArgResult::Parsed(Self {
                    #(#field_idents: #binding_names),*
                })
            }
        }
    };

    Ok(impl_tokens)
}

fn expand_enum(input: DeriveInput) -> syn::Result<TokenStream2> {
    let root = crate::picker_root();
    let enum_data = match &input.data {
        Data::Enum(data) => data,
        _ => unreachable!(),
    };

    let enum_name = &input.ident;

    if enum_data.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            enum_name,
            "derive enums require at least one variant",
        ));
    }

    let mut arms = Vec::new();
    for variant in &enum_data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                &variant.ident,
                "derive enum variants must not carry data",
            ));
        }

        let variant_ident = &variant.ident;
        let pascal_name = just_fmt::pascal_case!(variant_ident.to_string());
        arms.push(quote! {
            #pascal_name => #root::PickerArgResult::Parsed(Self::#variant_ident),
        });
    }

    let original_generics = &input.generics;
    let ty_generics = original_generics.split_for_impl().1;
    let original_params = &original_generics.params;
    let impl_generics = if original_params.is_empty() {
        quote! {}
    } else {
        quote! { <#original_params> }
    };

    let predicates: Vec<TokenStream2> = original_generics
        .where_clause
        .as_ref()
        .map(|wc| wc.predicates.iter().map(|pred| quote!(#pred)).collect())
        .unwrap_or_default();
    let where_clause = if predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#predicates),* }
    };

    let impl_tokens = quote! {
        impl #impl_generics #root::SinglePickable for #enum_name #ty_generics #where_clause {
            fn pick_single(str: Option<&str>) -> #root::PickerArgResult<Self> {
                let Some(raw) = str else {
                    return #root::PickerArgResult::NotFound;
                };

                let pascal = #root::__private::to_pascal_case(raw);
                match pascal.as_str() {
                    #(#arms)*
                    _ => #root::PickerArgResult::NotFound,
                }
            }
        }
    };

    Ok(impl_tokens)
}
