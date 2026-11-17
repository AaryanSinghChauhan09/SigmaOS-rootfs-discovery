// Integration test for SigmaOS-rootfs-discovery
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use rootfs_discovery as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
