use std::marker::PhantomData;
use std::ptr::NonNull;

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NvList {
    pub raw: *mut sys::nvlist_t,
    pub curr_nvpair: NvPair,
}

impl NvList {
    pub fn default() -> Self {
        NvList {
            raw: std::ptr::null_mut(),
            curr_nvpair: NvPair::default(),
        }
    }
}

impl IntoIterator for NvList {
    type Item = NvPair;
    type IntoIter = NvListIterator;

    fn into_iter(self) -> Self::IntoIter {
        NvListIterator {
            nvlist: self,
            curr_nvpair: NvPair::default(),
        }
    }
}
/*pub struct Iter<'a, T: 'a> {
    ptr: NonNull<NvList>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    #[inline]
    pub(super) fn new(slice: &'a [T]) -> Self {
        let ptr = slice.as_ptr();
        // SAFETY: Similar to `IterMut::new`.
        unsafe {
            assume(!ptr.is_null());

            let end = if mem::size_of::<T>() == 0 {
                (ptr as *const u8).wrapping_add(slice.len()) as *const T
            } else {
                ptr.add(slice.len())
            };

            Self {
                ptr: NonNull::new_unchecked(ptr as *mut T),
                end,
                _marker: PhantomData,
            }
        }
    }
}*/

#[derive(Clone, Debug, PartialEq)]
pub struct NvListIterator {
    pub nvlist: NvList,
    pub curr_nvpair: NvPair,
}

impl Iterator for NvListIterator {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.curr_nvpair.raw_nvpair =
                sys::nvlist_next_nvpair(self.nvlist.raw, self.curr_nvpair.raw_nvpair);

            match self.curr_nvpair.raw_nvpair.as_ref() {
                Some(_) => Some(self.curr_nvpair.clone()),
                None => None,
            }
        }
    }
}

pub struct Iter<'a, T: 'a> {
    ptr: NonNull<T>,
    end: *const T, // If T is a ZST, this is actually ptr+len.  This encoding is picked so that
    // ptr == end is a quick test for the Iterator being empty, that works
    // for both ZST and non-ZST.
    _marker: PhantomData<&'a T>,
}

/*impl Iterator for NvList {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.curr_nvpair.nvpair = sys::nvlist_next_nvpair(self.raw, self.curr_nvpair.nvpair);

            match self.curr_nvpair.nvpair.as_ref() {
                Some(_) => Some(self.curr_nvpair.clone()),
                None => None,
            }
        }
    }
}*/

pub enum NvFlag {
    UniqueName,
    UniqueNameType,
}
