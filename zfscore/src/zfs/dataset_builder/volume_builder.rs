use super::libnvpair;
pub struct VolumeBuilder {
    nvlist: Option<libnvpair::NvList>,
}

impl VolumeBuilder {
    pub fn new(name: impl AsRef<str>) -> Self {
        VolumeBuilder { nvlist: None }
    }
}
