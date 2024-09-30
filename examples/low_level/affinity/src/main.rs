// This example shows how to do core affinity in Rust
//
// References:
// - https://docs.rs/core_affinity/latest/core_affinity/

fn main() {
    extern crate core_affinity;

    use std::thread;

    // Retrieve the IDs of all active CPU cores.
    let core_ids = core_affinity::get_core_ids().unwrap();

    // Create a thread for each active CPU core.
    let handles = core_ids.into_iter().map(|core_id| {
        thread::spawn(move || {
            // Pin this thread to a single CPU core.
            let res = core_affinity::set_for_current(core_id);
            if res {
                println!("in thread {:?}", core_id);
            }
        })
    }).collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
