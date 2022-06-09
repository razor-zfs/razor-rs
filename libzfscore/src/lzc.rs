use super::*;

pub(crate) static LIBZFS_CORE: Lazy<Lzc> = Lazy::new(Lzc::init);

pub(crate) struct Lzc;

impl Lzc {
    fn init() -> Self {
        let _rc = unsafe { sys::libzfs_core_init() };
        Self
    }
}

impl Drop for Lzc {
    fn drop(&mut self) {
        unsafe { sys::libzfs_core_fini() }
    }
}
