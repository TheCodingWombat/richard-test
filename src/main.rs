mod auxiliary;

use auxiliary::frame_support::storage::types::map::StorageMap as Bar;
use auxiliary::frame_support::hash::Twox64Concat;

fn main() {
    println!("Hello, world!");

    let a = 3;

    if !true {
        print!("a");
    } else {
        print!("b");
    }

    auxiliary::missing_security_doc::test();
}

pub type Foo2<K, V> = Bar<Twox64Concat, K, V>;
