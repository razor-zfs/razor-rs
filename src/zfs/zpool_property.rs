mod ashift;
mod failmode;
mod health;
mod onoff;
mod yesno;

pub use super::property::{extract_from_bunch, InvalidProperty, Property};
use ashift::Ashift as AshiftSectorSize;
use failmode::Failmode as FailmodeStatus;
use health::Health as HealthStatus;
pub use onoff::OnOff;

pub type Allocated = Property<String>;
pub type Altroot = Property<String>;
pub type Ashift = Property<AshiftSectorSize>;
pub type Autoexpand = Property<OnOff>;
pub type Autoreplace = Property<OnOff>;
pub type Autotrim = Property<OnOff>;
pub type Bootfs = Property<String>; // TODO: check if it really string
pub type Cachefile = Property<String>; // TODO: check if it really string
pub type Capacity = Property<u64>;
pub type Comment = Property<String>;
pub type Delegation = Property<OnOff>;
pub type Dedupditto = Property<u64>;
pub type Expandsize = Property<u64>;
pub type Failmode = Property<FailmodeStatus>;
pub type Fragmentation = Property<u64>;
pub type Free = Property<u64>;
pub type Freeing = Property<u64>;
pub type Guid = Property<u64>;
pub type Health = Property<HealthStatus>;
pub type Listsnapshots = Property<OnOff>;
pub type Loadguid = Property<u64>;
pub type Multihost = Property<OnOff>;
pub type Readonly = Property<OnOff>;
pub type Size = Property<u64>;
pub type Version = Property<u64>;
