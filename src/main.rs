//! SigmaOS-rootfs-discovery — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_rootfs_discovery_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-rootfs-discovery: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-rootfs-discovery: ok");
}
