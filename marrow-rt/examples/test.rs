use std::collections::BTreeMap;

fn main() {
    let mut tree = BTreeMap::new();
    tree.insert(0, 3);
    tree.insert(3, 3);
    tree.insert(4, 4);
    tree.insert(5, 5);
    tree.insert(6, 6);
    tree.insert(7, 7);

    let a = tree.split_off(&1);

    println!("A: {:?}", tree);
    println!("B: {:?}", a);
}
