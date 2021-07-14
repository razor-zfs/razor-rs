use super::*;

impl<T> NvPair<T> {
    pub fn new(key: impl ToString, value: T) -> Self {
        let key = key.to_string();
        Self { key, value }
    }
}
