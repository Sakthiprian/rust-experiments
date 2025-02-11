use sysinfo::System;

pub fn get_memory_info(sys: &System) -> Vec<u64> {
    let mut memory_vec: Vec<u64> = Vec::new();
    let base: u64 = 10;

    memory_vec.push(sys.total_memory() / base.pow(6)); // Convert to MB
    memory_vec.push(sys.used_memory() / base.pow(6));
    memory_vec.push(sys.total_swap() / base.pow(6));
    memory_vec.push(sys.used_swap() / base.pow(6));

    memory_vec
}
