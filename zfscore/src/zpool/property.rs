pub use allocated::Allocated;

use super::InvalidProperty;

mod allocated;
mod altroot;
mod ashift;
mod bootfs;
mod cachefile;
mod expandsize;
mod failmode;
mod health;
mod onoff;
mod version;
mod yesno;

/*use allocated::Allocated as AllocatedSize;
use altroot::Altroot as AltrootDir;
use ashift::Ashift as AshiftSectorSize;
use bootfs::Bootfs as BootfsDataset;
use cachefile::Cachefile as CachefileLoc;
use expandsize::Expandsize as ExpandSize;
use failmode::Failmode as FailmodeStatus;
use health::Health as HealthStatus;
use onoff::OnOff;
use version::Version as PoolVersion;

pub type _Allocated = Property<AllocatedSize>;
pub type _Altroot = Property<AltrootDir>;
pub type _Ashift = Property<AshiftSectorSize>;
pub type _Autoexpand = Property<OnOff>;
pub type _Autoreplace = Property<OnOff>;
pub type _Autotrim = Property<OnOff>;
pub type _Bootfs = Property<BootfsDataset>;
pub type _Cachefile = Property<CachefileLoc>;
pub type _Capacity = Property<u64>;
pub type _Comment = Property<String>;
pub type _Delegation = Property<OnOff>;
pub type _Dedupditto = Property<u64>;
pub type _Expandsize = Property<ExpandSize>;
pub type _Failmode = Property<FailmodeStatus>;
pub type _Fragmentation = Property<u64>;
pub type _Free = Property<u64>;
pub type _Freeing = Property<u64>;
pub type _Guid = Property<u64>;
pub type _Health = Property<HealthStatus>;
pub type _Listsnapshots = Property<OnOff>;
pub type _Loadguid = Property<u64>;
pub type _Multihost = Property<OnOff>;
pub type _Readonly = Property<OnOff>;
pub type _Size = Property<u64>;
pub type _Version = Property<PoolVersion>;
pub type _Name = Property<String>;
*/