//a Imports
use std::pin::Pin;

use super::VecWithIndex;
use crate::make_index;

//a Name
make_index!(StringIndex, usize);

//a StringsWithIndex
//tp StringsWithIndex
#[derive(Debug, Default)]
pub struct StringsWithIndex<'swi> {
    strings: VecWithIndex<'swi, &'swi str, StringIndex, Pin<String>>,
}

//ip Index<Name> for StringsWithIndex
impl std::ops::Index<StringIndex> for StringsWithIndex<'_> {
    type Output = str;
    fn index(&self, p: StringIndex) -> &str {
        Pin::into_inner(self.strings[p].as_ref())
    }
}

//ip StringsWithIndex
impl<'swi> StringsWithIndex<'swi> {
    //mi add
    /// Add string to the indexed array; must only be issued if find returns false
    fn add<S: Into<String>>(&mut self, s: S) -> StringIndex {
        let s = s.into();
        let pinned_string = Pin::new(s);
        let pinned_str: &str = unsafe { std::mem::transmute::<_, _>(pinned_string.as_ref()) };
        self.strings.find_or_add(pinned_str, |_| pinned_string).1
    }

    //mp find_or_add
    #[must_use]
    pub fn find_or_add<S: Into<String> + AsRef<str>>(&mut self, s: S) -> (bool, StringIndex) {
        if let Some(p) = self.find_string(s.as_ref()) {
            (true, p)
        } else {
            (false, self.add(s))
        }
    }

    //mp insert
    /// Add data to the array and index, but only if it is not present
    ///
    /// If it is already present, return an Err
    pub fn insert<S: Into<String> + AsRef<str>>(
        &mut self,
        s: S,
    ) -> Result<StringIndex, StringIndex> {
        let (found, index) = self.find_or_add(s);
        if found {
            Err(index)
        } else {
            Ok(index)
        }
    }

    //mp find_string
    pub fn find_string(&self, s: &'swi str) -> Option<StringIndex> {
        self.strings.find_key(&s)
    }

    //mp strings
    /// Iterate through the keys
    pub fn strings(&self) -> impl Iterator<Item = &&str> {
        self.strings.keys()
    }

    //mp contains
    /// Returns true if this contains a string
    pub fn contains<S: AsRef<str>>(&self, s: &S) -> bool {
        self.strings.contains(&s.as_ref())
    }

    //mp fmt_string
    pub fn fmt_string(
        &self,
        fmt: &mut std::fmt::Formatter,
        name: StringIndex,
    ) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self[name])
    }
}
