mod binary;
pub use self::binary::BinaryTree;

pub trait Tree<T> {
    fn insert(&mut self, value: T);
    fn remove(&mut self, value: T);
    fn exists(&self, value: T) -> bool;
    fn inorder(&self) -> Vec<T> where T: Copy;
}