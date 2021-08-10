use std::fmt;

impl fmt::Debug for crate::NvPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NvPair")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl fmt::Debug for crate::NvList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.items()).finish()
    }
}
