use super::*;

mod impls;
mod impls_trait;

pub struct NvPair<T> {
    pub key: String,
    pub value: T,
}
