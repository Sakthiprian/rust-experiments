use sysinfo::{System, Pid, DiskUsage};

pub fn get_process_info(sys: &System) -> Vec<(Pid, String, f32, DiskUsage)> {
    let mut process_vec = Vec::new();
    
    for (pid, process) in sys.processes() {
        process_vec.push((
            *pid, // Dereference &Pid to get Pid (by value)
            process.name().to_string_lossy().into_owned(),
            process.cpu_usage(),
            process.disk_usage(),
        ));
    }

    process_vec
}
