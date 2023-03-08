use kaantor_tree::{KItem, KNode, KTree, Pretty};

pub struct MyItem {
    inner: usize,
}

impl KItem for MyItem {
    type Key = usize;

    fn key(&self) -> &Self::Key {
        &self.inner
    }
}

impl From<usize> for MyItem {
    fn from(value: usize) -> Self {
        Self { inner: value }
    }
}

fn main() {
    let n1 = KNode::<MyItem>::new(1.into(), vec![2, 3]);
    let n2 = KNode::<MyItem>::new(2.into(), vec![4, 5]);
    let n3 = KNode::<MyItem>::new(3.into(), vec![6]);
    let n4 = KNode::<MyItem>::new(4.into(), vec![]);
    let n5 = KNode::<MyItem>::new(5.into(), vec![7]);
    let n6 = KNode::<MyItem>::new(6.into(), vec![]);
    let n7 = KNode::<MyItem>::new(7.into(), vec![]);

    let tree = KTree::new(1, vec![n1, n2, n3, n4, n5, n6, n7]);
    tree.print("MY TREE");
}
