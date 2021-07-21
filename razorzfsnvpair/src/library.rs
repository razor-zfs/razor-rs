use razorzfsnvpair_sys as sys;

use super::NvListError;
use name_serializer::NameSerializer;
use nvpair::{ContextType, NvPair};

mod name_serializer;
mod nvlist_deserializer;
mod nvlist_serializer;
mod nvpair;
mod traits;

pub type Result<T> = std::result::Result<T, NvListError>;
