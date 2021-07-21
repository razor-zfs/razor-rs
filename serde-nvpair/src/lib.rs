use razorzfsnvpair as libnvpair;

mod de;
mod error;
mod ser;

pub type Result<T> = std::result::Result<T, libnvpair::NvListError>;

fn main() {
    println!("Hello, world!");
}
