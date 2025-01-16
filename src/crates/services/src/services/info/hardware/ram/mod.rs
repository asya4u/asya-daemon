use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub fn get_total_memory(_: String) -> Option<u64> {
    system(|sys| Some(sys.total_memory()))
}

pub fn get_used_memory(_: String) -> Option<u64> {
    system(|sys| Some(sys.used_memory()))
}

pub fn get_free_memory(_: String) -> Option<u64> {
    system(|sys| Some(sys.free_memory()))
}

pub fn get_available_memory(_: String) -> Option<u64> {
    system(|sys| Some(sys.available_memory()))
}

pub fn get_total_swap(_: String) -> Option<u64> {
    system(|sys| Some(sys.total_swap()))
}

pub fn get_used_swap(_: String) -> Option<u64> {
    system(|sys| Some(sys.used_swap()))
}

pub fn get_free_swap(_: String) -> Option<u64> {
    system(|sys| Some(sys.free_swap()))
}

fn system<T, F>(f: T) -> Option<F>
where
    T: FnOnce(&sysinfo::System) -> Option<F>,
{
    f(&System::new_with_specifics(RefreshKind::everything().with_memory(
        MemoryRefreshKind::everything().with_ram().with_swap(),
    )))
}
