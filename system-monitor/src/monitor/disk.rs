use sysinfo::Disks;

pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

pub fn get_disk_info() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    disks
        .iter()
        .map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            
            DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total,
                available,
                used,
            }
        })
        .collect()
}
