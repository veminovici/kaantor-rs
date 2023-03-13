use ptree::{print_tree, TreeBuilder};
use std::fmt::Debug;

pub trait Pretty {
    fn print(&self, title: &str);
}

pub trait KItem {
    type Key;

    fn key(&self) -> &Self::Key;
}

pub struct KNode<I>
where
    I: KItem,
{
    value: I,
    children: Vec<I::Key>,
}

impl<I> KNode<I>
where
    I: KItem,
{
    pub fn new(value: I, children: Vec<I::Key>) -> Self {
        Self { value, children }
    }

    pub fn key(&self) -> &I::Key {
        self.value.key()
    }

    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn add_child(&mut self, child: &Self)
    where
        I::Key: Copy,
    {
        self.children.extend(&child.children);
        self.children.extend(vec![child.key()]);
    }
}

pub struct KTree<I>
where
    I: KItem,
{
    root: I::Key,
    nodes: Vec<KNode<I>>,
}

impl<I> KTree<I>
where
    I: KItem,
{
    pub fn new(root: I::Key, nodes: Vec<KNode<I>>) -> Self {
        Self { root, nodes }
    }
}

impl<I> KTree<I>
where
    I: KItem,
    I::Key: Debug + PartialEq + Eq,
{
    fn prety_child<'a>(
        &'a self,
        node: &'a KNode<I>,
        tb: &'a mut TreeBuilder,
    ) -> &'a mut TreeBuilder {
        let text = format!("{:?}", node.key());
        let tb = tb.begin_child(text);
        let tb = node
            .children
            .iter()
            .fold(tb, |tb, key| self.pretty_node(key, tb));
        tb.end_child()
    }

    pub(crate) fn pretty_node<'a>(
        &'a self,
        key: &'a I::Key,
        tb: &'a mut TreeBuilder,
    ) -> &'a mut TreeBuilder {
        if let Some(node) = self.nodes.iter().find(|node| node.key() == key) {
            if node.is_leaf() {
                tb.add_empty_child(format!("{:?}", key))
            } else {
                self.prety_child(node, tb)
            }
        } else {
            tb
        }
    }
}

impl<I> Pretty for KTree<I>
where
    I: KItem,
    I::Key: Debug + PartialEq + Eq,
{
    fn print(&self, title: &str) {
        let mut tb = TreeBuilder::new(title.to_string());
        let tree = self.pretty_node(&self.root, &mut tb).build();
        print_tree(&tree).unwrap();
    }
}
