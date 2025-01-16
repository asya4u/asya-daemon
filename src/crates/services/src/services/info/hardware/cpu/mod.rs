use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub fn get_global_usage(_: String) -> Option<f32> {
    system(|sys| Some(sys.global_cpu_usage()))
}

pub fn get_brand(_: String) -> Option<String> {
    system(|sys| Some(sys.cpus()[0].brand().to_string()))
}

pub fn get_core_count(_: String) -> Option<usize> {
    system(|sys| Some(sys.cpus().len()))
}

pub fn get_vendor(_: String) -> Option<String> {
    system(|sys| Some(sys.cpus()[0].vendor_id().to_string()))
}

pub fn get_name(_: String) -> Option<String> {
    system(|sys| Some(sys.cpus()[0].name().to_string()))
}

pub fn get_frequency(_: String) -> Option<u64> {
    system(|sys| Some(sys.cpus()[0].frequency()))
}

fn system<T>(f: impl FnOnce(&sysinfo::System) -> Option<T>) -> Option<T> {
    let sys = System::new_with_specifics(RefreshKind::everything().with_cpu(CpuRefreshKind::everything()));
    f(&sys)
}
