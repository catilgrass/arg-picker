use crate::PickerArgs;

/// One argument that no pick consumed, together with where it was found.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UnmatchedArg {
    /// The index of the word in the [`PickerArgs`] the picker was built from.
    ///
    /// The count starts at zero and excludes the program name. Picking
    /// [`UNMATCHED`](crate::UNMATCHED) reports this index so a caller can point at the position
    /// the word held in the original list.
    pub index: usize,

    /// The word as it was written.
    pub raw: String,
}

/// The arguments that no pick consumed.
///
/// This is [`REMAINS`](crate::REMAINS) with positions: each entry keeps the index it had in the
/// argument list, so a caller can report not only *what* was unexpected but *where* it was. The
/// end-of-options marker (`--`) is not an entry, since it is syntax rather than an argument; the
/// words after it are entries like any other.
///
/// The entries are in the order the arguments appeared, so their indices are ascending.
///
/// Iterating yields [`UnmatchedArg`] (or `&UnmatchedArg` when iterating a reference). The only
/// lossy conversion is into [`PickerArgs`], which keeps the words and drops the positions; no
/// conversion to `Vec<String>` or other word collections is provided, so the choice to drop the
/// positions is made explicitly.
///
/// The complete argument list the picker was built from is kept as well, so a caller can show the
/// original input next to the positions in the entries.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UnmatchedArgs {
    entries: Vec<UnmatchedArg>,
    original: Vec<String>,
}

impl UnmatchedArgs {
    /// Builds the value from entries the pick phase produced and the list they point into.
    pub(crate) const fn new(entries: Vec<UnmatchedArg>, original: Vec<String>) -> Self {
        Self { entries, original }
    }

    /// Returns the complete argument list the picker was built from, before any matching.
    ///
    /// The program name is not part of it, and the list is copied, so it does not borrow from the
    /// parse. The indices of [`UnmatchedArg`] are positions in this list.
    #[must_use]
    pub fn original(&self) -> &[String] {
        &self.original
    }

    /// Returns the number of unmatched arguments.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if every argument was matched.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates over the unmatched arguments.
    pub fn iter(&self) -> std::slice::Iter<'_, UnmatchedArg> {
        self.entries.iter()
    }
}

impl<'a> IntoIterator for &'a UnmatchedArgs {
    type Item = &'a UnmatchedArg;
    type IntoIter = std::slice::Iter<'a, UnmatchedArg>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl IntoIterator for UnmatchedArgs {
    type Item = UnmatchedArg;
    type IntoIter = std::vec::IntoIter<UnmatchedArg>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl From<UnmatchedArgs> for PickerArgs<'_> {
    fn from(value: UnmatchedArgs) -> Self {
        Self::Owned(value.entries.into_iter().map(|entry| entry.raw).collect())
    }
}
