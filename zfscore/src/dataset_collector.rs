// //use chrono::prelude::*;

// use std::ffi::CString;

// use crate::libzfs;
// use crate::ZfsDatasetHandle;
// use razor_zfscore_sys as sys;

// #[derive(Debug)]
// pub struct DatasetCollectorBuilder {
//     from_dataset: Option<String>,
//     datasets: Vec<ZfsDatasetHandle>,
//     r#type: libzfs::zfs_type_t,
//     recursive: Option<bool>,
// }

// impl DatasetCollectorBuilder {
//     pub(crate) fn new() -> Self {
//         Self {
//             from_dataset: None,
//             datasets: Vec::new(),
//             r#type: libzfs::zfs_type_t(0),
//             recursive: None,
//         }
//     }

//     pub(crate) fn from(dataset: impl AsRef<str>) -> Self {
//         Self {
//             from_dataset: Some(dataset.as_ref().to_owned()),
//             datasets: Vec::new(),
//             r#type: libzfs::zfs_type_t(0),
//             recursive: None,
//         }
//     }

//     pub fn filesystems(mut self) -> Self {
//         self.r#type |= libzfs::zfs_type_t::ZFS_TYPE_FILESYSTEM;

//         self
//     }

//     pub fn volumes(mut self) -> Self {
//         self.r#type |= libzfs::zfs_type_t::ZFS_TYPE_VOLUME;

//         self
//     }

//     pub fn _snapshots(mut self) -> Self {
//         self.r#type |= libzfs::zfs_type_t::ZFS_TYPE_SNAPSHOT;

//         self
//     }

//     pub fn _bookmarks(mut self) -> Self {
//         self.r#type |= libzfs::zfs_type_t::ZFS_TYPE_BOOKMARK;

//         self
//     }

//     pub fn recursive(mut self) -> Self {
//         self.recursive = Some(true);

//         self
//     }

//     fn add_dataset(&mut self, handle: *mut sys::zfs_handle_t) {
//         let converted_handle = ZfsDatasetHandle::from(handle);

//         let mut passed_filter = false;

//         if self.r#type & converted_handle.r#type() != libzfs::zfs_type_t(0) {
//             passed_filter = true;
//         }

//         if passed_filter {
//             self.datasets.push(converted_handle);
//         }

//         if let Some(_) = self.recursive {
//             unsafe {
//                 sys::zfs_iter_filesystems(
//                     handle,
//                     Some(zfs_list_cb),
//                     &mut self.datasets as *mut _ as *mut libc::c_void,
//                 );
//             }
//         }
//     }

//     pub fn get_collection(mut self) -> DatasetCollector {
//         if let Some(name) = self.from_dataset {
//             let cname = CString::new(name).unwrap();
//             unsafe {
//                 let handle = libzfs::zfs_open(cname.as_ptr());
//                 if sys::zfs_get_type(handle) != libzfs::zfs_type_t::ZFS_TYPE_VOLUME {
//                     sys::zfs_iter_filesystems(
//                         handle,
//                         Some(zfs_list_cb),
//                         &mut self.datasets as *mut _ as *mut libc::c_void,
//                     );
//                 }
//             }
//         } else {
//             unsafe {
//                 libzfs::zfs_iter_root(
//                     zfs_list_cb,
//                     &mut self.datasets as *mut _ as *mut libc::c_void,
//                 )
//             };
//         }

//         DatasetCollector::new(self.datasets)
//     }
// }

// #[derive(Debug)]
// pub struct DatasetCollector {
//     datasets: Vec<ZfsDatasetHandle>,
// }

// impl DatasetCollector {
//     pub(crate) fn new(datasets: Vec<ZfsDatasetHandle>) -> Self {
//         Self { datasets }
//     }
// }

// impl IntoIterator for DatasetCollector {
//     type Item = ZfsDatasetHandle;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.datasets.into_iter()
//     }
// }

// #[no_mangle]
// extern "C" fn zfs_list_cb(handle: *mut sys::zfs_handle_t, ptr: *mut libc::c_void) -> libc::c_int {
//     let data: &mut DatasetCollectorBuilder = unsafe { &mut *(ptr as *mut DatasetCollectorBuilder) };
//     data.add_dataset(handle);

//     0
// }
