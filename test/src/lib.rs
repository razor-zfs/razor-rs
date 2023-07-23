// Make sure to run
// echo 3 | sudo tee /sys/module/zfs/parameters/zvol_volmode
// before running this test.

use std::env;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

use nanoid::nanoid;
use nanoid_dictionary::ALPHANUMERIC;

use razor_safe_lzc as lzc;
use razor_zfs as zfs;

use zfs::zfs::property::MountPoint;
use zfs::Filesystem;
use zfs::Zfs;

#[derive(Debug)]
pub struct TestNamespace {
    pool: String,
    namespace: Filesystem,
    sync: bool,
    delay: Duration,
}

impl TestNamespace {
    const POOL: &'static str = "rpool";
    const TOP_LEVEL_CONTAINER: &'static str = "razor-test";

    pub fn unique() -> Self {
        Self::init();
        let (pool, container) = Self::pool_and_container();

        let namespace = format!("{}/{}/{}", pool, container, nanoid!(8, ALPHANUMERIC));
        let namespace = Zfs::filesystem()
            .canmount(false)
            .mountpoint(MountPoint::None)
            .create(&namespace)
            .unwrap();
        let sync = false;
        let delay = Duration::from_millis(0);
        Self {
            pool,
            namespace,
            sync,
            delay,
        }
    }

    pub fn sync_delay(&self) {
        if self.sync {
            lzc::sync_pool(&self.pool, true).unwrap();
        }
        sleep(self.delay);
    }

    pub fn unique_name(&self) -> String {
        format!("{}/{}", self.namespace.name(), nanoid!(8, ALPHANUMERIC))
    }
    fn init() {
        fs::write("/sys/module/zfs/parameters/zvol_volmode", "3")
            .expect("Failed to set zvol_volmod to 3");
    }

    fn pool_and_container() -> (String, String) {
        if let Ok(name) = env::var("RAZOR_TEST") {
            if let Some((pool, container)) = name.split_once('/') {
                return (pool.to_string(), container.to_string());
            }
        }

        (
            Self::POOL.to_string(),
            Self::TOP_LEVEL_CONTAINER.to_string(),
        )
    }
}

impl Drop for TestNamespace {
    fn drop(&mut self) {
        self.namespace.destroy_recursive().unwrap()
    }
}
