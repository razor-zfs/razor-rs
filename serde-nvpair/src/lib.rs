use razorzfsnvpair as libnvpair;

mod de;
mod ser;

pub type Result<T> = std::result::Result<T, libnvpair::NvListError>;
