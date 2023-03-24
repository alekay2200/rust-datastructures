mod tree;
pub use tree::BinaryTree;

pub trait Datastructure<T> {
    fn insert(&mut self, value: T);
    fn remove(&mut self, value: T);
    fn exists(&self, value: T) -> bool;
}