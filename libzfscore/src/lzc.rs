use super::*;

pub(crate) static LIBZFS_CORE: Lazy<Lzc> = Lazy::new(Lzc::init);

pub(crate) struct Lzc;

impl Lzc {
    fn init() -> Self {
        let _rc = unsafe { sys::libzfs_core_init() };
        libzfs::zfs_version().ensure_compatible();
        Self
    }
}
