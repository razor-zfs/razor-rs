use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::Not;
use std::ptr;

use razor_libnvpair as libnvpair;

pub use self::access::NvListAccess;

use crate::error::value_or_err;

use super::*;

mod access;

#[derive(PartialEq)]
pub struct NvList {
    nvl: *mut libnvpair::nvlist_t,
}

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

impl access::NvListAccess for NvList {
    fn nvl(&self) -> *mut libnvpair::nvlist_t {
        self.nvl
    }
}

impl<'a, T> access::NvListAccess for NvListRef<'a, T> {
    fn nvl(&self) -> *mut libnvpair::nvlist_t {
        self.nvl
    }
}

impl NvList {
    pub fn new(flag: NvFlag) -> Self {
        let flag = match flag {
            NvFlag::UniqueName => libnvpair::NV_UNIQUE_NAME,
            NvFlag::UniqueNameType => libnvpair::NV_UNIQUE_NAME_TYPE,
        };
        let nvl = unsafe { libnvpair::nvlist_alloc(flag) };
        Self { nvl }
    }
}

impl From<*mut libnvpair::nvlist_t> for NvList {
    fn from(nvl: *mut libnvpair::nvlist_t) -> Self {
        Self { nvl }
    }
}

impl Clone for NvList {
    fn clone(&self) -> Self {
        let nvl = unsafe { libnvpair::nvlist_dup(self.nvl) };
        Self::from(nvl)
    }
}

impl Drop for NvList {
    fn drop(&mut self) {
        unsafe { libnvpair::nvlist_free(self.nvl) };
    }
}

impl IntoIterator for NvList {
    type Item = NvPair;
    type IntoIter = NvListIterator;

    fn into_iter(self) -> Self::IntoIter {
        NvListIterator {
            nvlist: self,
            nvp: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NvListIterator {
    nvlist: NvList,
    nvp: Option<*mut libnvpair::nvpair_t>,
}

impl Iterator for NvListIterator {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        let nvp = self.nvp.unwrap_or_else(ptr::null_mut);
        let nvp = unsafe { libnvpair::nvlist_next_nvpair(self.nvlist.nvl, nvp) };
        self.nvp = nvp.is_null().not().then(|| nvp);
        self.nvp.map(NvPair::from)
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
        let mut nvlist = NvList::new(NvFlag::UniqueName);
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
