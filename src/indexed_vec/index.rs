//a Idx
//tt Idx
pub trait Idx:
    Copy + std::fmt::Debug + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash + 'static
{
    const NONE: Option<usize>;
    fn none() -> Self {
        assert!(Self::NONE.is_some());
        Self::from_usize(Self::NONE.unwrap())
    }
    fn is_none(self) -> bool {
        (Self::NONE)
            .and_then(|v| if self.index() != v { None } else { Some(true) })
            .unwrap_or(false)
    }
    fn opt_index(self) -> Option<usize> {
        let v = self.index();
        if let Some(none) = Self::NONE {
            if v == none {
                None
            } else {
                Some(v)
            }
        } else {
            Some(v)
        }
    }
    fn from_usize(idx: usize) -> Self;
    fn index(self) -> usize;
}

//a Macro make_index
#[macro_export]
macro_rules! make_index {
    {$id: ident, $t:ty, $none:expr} => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize)]
        pub struct $id($t);

        impl $crate :: indexed_vec :: Idx for $id {
            const NONE : Option<usize> =  $none ;
            fn from_usize(n: usize) -> Self { Self(n as usize)}
            fn index(self) -> usize {
                self.0 as usize
            }
        }
        impl $id {
            #[allow(dead_code)]
            #[track_caller]
            pub fn decrement(self) -> Self {
                assert!(self.0 != 0, "Decrement of index from 0 is illegal");
                Self(self.0 - 1)
            }
            #[allow(dead_code)]
            pub fn increment(self) -> Self {
                Self(self.0 + 1)
            }
        }
        impl std::fmt::Display for $id {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                self.0.fmt(fmt)
            }
        }
    };
    {$id: ident, $t:ty} => {
        make_index! { $id, $t, None }
    };
}
