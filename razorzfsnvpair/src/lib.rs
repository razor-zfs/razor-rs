pub use razorzfsnvpair_sys as sys;

pub use error::NvListError;
pub use library::ContextType;
pub use library::CtxIter;
pub use library::NvFlag;
pub use library::NvList;
pub use library::NvListIterator;
pub use library::NvPair;
pub use library::NvPairType;
pub use library::SafeNvPair;
use sys::nvpair_t;

mod error;
mod library;

pub type Result<T> = std::result::Result<T, NvListError>;
