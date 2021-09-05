use std::fmt;

use crate::{to_value, NvList, NvListAccess, NvListRef, NvPair};

impl fmt::Debug for NvPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name();
        let value = to_value(self);
        f.debug_struct("NvPair")
            .field("name", &name)
            .field("value", &value)
            .finish()
    }
}

impl fmt::Debug for NvList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.items()).finish()
    }
}

impl<'a, T> fmt::Debug for NvListRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.items()).finish()
    }
}
