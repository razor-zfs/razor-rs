use std::ffi::CString;
use std::marker::{PhantomData, Send};
use std::ops::{Deref, DerefMut, Not};
use std::ptr;

use razor_libnvpair as libnvpair;

use super::*;

mod impls;

/// Safe idiomatic nvlist_t wrapper. Use it when you need to create your own nvlist.
/// Cleanly frees underlying memory when dropped.
///
#[derive(PartialEq, Eq)]
pub struct NvList {
    nvl: *mut libnvpair::nvlist_t,
}

/// Safe idiomatic nvlist_t wrapper. Use it when you need access to nvlist_t that is NOT owned by you.
/// It tracks the lifetime of its parent object and does not outlive it.
///
#[derive(Clone)]
pub struct NvListRef<'a, T> {
    nvl: *mut libnvpair::nvlist_t,
    anchor: PhantomData<&'a T>,
}

impl<'a, T> NvListRef<'a, T> {
    pub fn from_raw(nvl: *mut libnvpair::nvlist_t, _anchor: &'a T) -> Self {
        Self {
            nvl,
            anchor: PhantomData,
        }
    }
}

impl NvList {
    /// Create new empty nvlist object
    pub fn new() -> Self {
        let nvl = unsafe { libnvpair::fnvlist_alloc() };
        Self { nvl }
    }
}

impl Default for NvList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<*mut libnvpair::nvlist_t> for NvList {
    fn from(nvl: *mut libnvpair::nvlist_t) -> Self {
        Self { nvl }
    }
}

impl Drop for NvList {
    fn drop(&mut self) {
        unsafe { libnvpair::fnvlist_free(self.nvl) };
    }
}

impl Deref for NvList {
    type Target = *mut libnvpair::nvlist_t;

    fn deref(&self) -> &Self::Target {
        &self.nvl
    }
}

impl DerefMut for NvList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nvl
    }
}

impl AsRef<*mut libnvpair::nvlist_t> for NvList {
    fn as_ref(&self) -> &*mut libnvpair::nvlist_t {
        &self.nvl
    }
}

impl<'a, T> AsRef<*mut libnvpair::nvlist_t> for NvListRef<'a, T> {
    fn as_ref(&self) -> &*mut libnvpair::nvlist_t {
        &self.nvl
    }
}

impl IntoIterator for NvList {
    type Item = NvPair;
    type IntoIter = NvListIterator;

    fn into_iter(self) -> Self::IntoIter {
        NvListIterator {
            nvlist: self,
            nvpair: None,
        }
    }
}

unsafe impl Send for NvList {}

#[derive(Debug)]
pub struct NvListIterator {
    nvlist: NvList,
    nvpair: Option<NvPair>,
}

impl Iterator for NvListIterator {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        let nvpair = self.nvpair.unwrap_or_else(Self::Item::null);
        let nvp = unsafe { libnvpair::nvlist_next_nvpair(*self.nvlist, *nvpair) };
        self.nvpair = if !nvp.is_null() {
            Some(NvPair::from(nvp))
        } else {
            None
        };
        self.nvpair
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    nvl: *mut libnvpair::nvlist_t,
    nvp: Option<*mut libnvpair::nvpair_t>,
    anchor: PhantomData<&'a NvList>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        let nvp = self.nvp.unwrap_or_else(ptr::null_mut);
        let nvp = unsafe { libnvpair::nvlist_next_nvpair(self.nvl, nvp) };
        self.nvp = nvp.is_null().not().then(|| nvp);
        self.nvp.map(NvPair::from)
    }
}

#[derive(Debug)]
pub struct Items<'a> {
    nvl: *mut libnvpair::nvlist_t,
    nvp: Option<*mut libnvpair::nvpair_t>,
    anchor: PhantomData<&'a NvList>,
}

impl<'a> Iterator for Items<'a> {
    type Item = (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        let nvp = self.nvp.unwrap_or_else(ptr::null_mut);
        let nvp = unsafe { libnvpair::nvlist_next_nvpair(self.nvl, nvp) };
        self.nvp = nvp.is_null().not().then(|| nvp);
        self.nvp
            .map(NvPair::from)
            .map(|nvpair| (nvpair.name().to_string(), to_value(&nvpair)))
    }
}

#[derive(Debug)]
pub enum NvFlag {
    UniqueName,
    UniqueNameType,
}

#[cfg(test)]
mod tests {
    use super::*;

    use libnvpair::data_type_t::*;

    #[test]
    fn nvlist_iter() {
        let mut nvlist = NvList::new();
        let arr = [1, 2, 3, 4, 5];
        nvlist.add_uint16("a", 3).unwrap();
        nvlist.add_uint32("b", 5).unwrap();
        nvlist.add_uint8_array("d", &arr).unwrap();

        let mut iter = dbg!(nvlist).into_iter();
        let pair1 = dbg!(iter.next().unwrap());
        let pair2 = dbg!(iter.next().unwrap());
        let pair3 = dbg!(iter.next().unwrap());
        assert_eq!(pair1.name(), "a");
        assert_eq!(pair1.r#type(), DATA_TYPE_UINT16);
        assert_eq!(pair2.name(), "b");
        assert_eq!(pair2.r#type(), DATA_TYPE_UINT32);
        assert_eq!(pair3.name(), "d");
        assert_eq!(pair3.r#type(), DATA_TYPE_UINT8_ARRAY);
        assert_eq!(None, iter.next());
    }
}
