// aux-build:frame-support.rs

use crate::auxiliary::frame_support::storage::types::map::StorageMap as Bar;
use crate::auxiliary::frame_support::hash::Twox64Concat;

/// # Security
///
/// Twox64Concat is allowed because this is a test
pub type Foo<K, V> = Bar<Twox64Concat, K, V>;

pub type Foo2<K, V> = Bar<Twox64Concat, K, V>;

fn main() {}

pub fn test() {
    println!("test");
}
