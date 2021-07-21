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

impl Iterator for NvList {
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
}

pub enum NvFlag {
    UniqueName,
    UniqueNameType,
}
