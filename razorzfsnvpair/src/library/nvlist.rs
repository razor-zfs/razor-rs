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
            completed: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NvListIterator {
    pub nvlist: NvList,
    pub curr_nvpair: NvPair,
    pub completed: bool,
}

impl Iterator for NvListIterator {
    type Item = NvPair;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if !self.completed {
                self.curr_nvpair.raw_nvpair =
                    sys::nvlist_next_nvpair(self.nvlist.raw, self.curr_nvpair.raw_nvpair);

                match self.curr_nvpair.raw_nvpair.as_ref() {
                    Some(_) => Some(self.curr_nvpair.clone()),
                    None => {
                        self.completed = true;
                        None
                    }
                }
            } else {
                None
            }
        }
    }
}

pub enum NvFlag {
    UniqueName,
    UniqueNameType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvlist_iter() {
        let mut nvlist = nvlist_alloc(NvFlag::UniqueName).unwrap();
        let arr: [u8; 5] = [1, 2, 3, 4, 5];
        nvlist_add_uint16(&nvlist, "a", 3).unwrap();
        nvlist_add_uint32(&nvlist, "b", 5).unwrap();
        nvlist_add_uint8_arr(&nvlist, "d", arr).unwrap();
        let mut nvpair = NvPair::default();
        let mut iter = nvlist.into_iter();
        let pair1 = iter.next().unwrap();
        let pair2 = iter.next().unwrap();
        let pair3 = iter.next().unwrap();
        assert_eq!("a".to_string(), pair1.name().unwrap());
        assert_eq!(NvPairType::Uint16, pair1.r#type());
        assert_eq!("b".to_string(), pair2.name().unwrap());
        assert_eq!(NvPairType::Uint32, pair2.r#type());
        assert_eq!("d".to_string(), pair3.name().unwrap());
        assert_eq!(NvPairType::Uint8Array, pair3.r#type());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
