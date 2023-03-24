use std::fmt::Debug;
use std::ptr;
use crate::Datastructure;

type NodeRef<T> = Option<*mut Node<T>>;

#[derive(Debug)]
struct Node<T> where T: PartialEq + PartialOrd {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}


impl<T> Node<T> where T: PartialEq + PartialOrd {
    // Allocate new node and return the raw pointer
    fn new(value: T) -> *mut Node<T> {
        // Allocate memory on the heap
        Box::into_raw(Box::new(Node { value, left: None, right: None }))
        // To deallocate the memory convert te raw pointer back to a box an call drop mehtod drop(Box::form_raw(raw_node_pointer))
    }

    // Insert new node using recursive approach
    fn insert_rec(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(node) => unsafe { node.as_mut().unwrap().insert_rec(value) },
                None => self.left = Some(Node::new(value))
            }
        } else if value > self.value {
            match self.right {
                Some(node) => unsafe { node.as_mut().unwrap().insert_rec(value) },
                None => self.right = Some(Node::new(value))
            }
        // Value already exists, dont insert
        } else { }
    }

    // Return true if the node dont have any child
    fn is_leaft(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn has_one_child(&self) -> bool {
        (self.left.is_some() && self.right.is_none()) || (self.left.is_none() && self.right.is_some())
    }

    fn exists(&self, value: T) -> bool {
        if self.value == value {
            return true;
        } else {
            if value < self.value {
                match self.left {
                    Some(node) => unsafe { (*node).exists(value) },
                    None => false
                }
            } else {
                match self.right {
                    Some(node) => unsafe { (*node).exists(value) },
                    None => false
                }
            }
        }
    }

    fn inorder_rec(&self, values: &mut Vec<T>) where T: Copy {
        match self.left {
            Some(node) => unsafe { node.as_ref().unwrap().inorder_rec(values) },
            None => {}
        };

        values.push(self.value);

        match self.right {
            Some(node) => unsafe { node.as_ref().unwrap().inorder_rec(values) },
            None => {}
        };
    }

    // fn inorder(&self) -> Vec<T> where T: Copy {
        
    // }

    // fn exists(&self, value: T) -> bool {
        
    // }

    fn print_tree_recursive(&self, level: usize) where T: Debug {
        match self.right {
            Some(node) => unsafe { node.as_ref().unwrap().print_tree_recursive(level + 1) },
            None => {}
        }

        for _ in 0..level {
            print!("\t");
        }
        println!("{:?}", self.value);

        match self.left {
            Some(node) => unsafe { node.as_ref().unwrap().print_tree_recursive(level + 1) },
            None => {}
        }
    }
}

pub struct BinaryTree<T> where T: PartialEq + PartialOrd {
    root: NodeRef<T>,
    size: usize
}


impl<T> BinaryTree<T> where T: PartialEq + PartialOrd {
    pub fn new() -> Self {
        BinaryTree { root: None, size: 0 }
    }

    pub fn insert_rec(&mut self, value: T) {
        match self.root {
            Some(node) => unsafe { node.as_mut().unwrap().insert_rec(value) },
            None => self.root = Some(Node::new(value))
        }
        self.size += 1;
    }

    fn find_smallest(&self, node: *mut Node<T>) -> *mut Node<T> {
        let mut smallest = node; 
        unsafe {
            while let Some(s) = (*smallest).left {
                smallest = s;
            }
        }
        return smallest;
    }

    pub fn inorder(&self) -> Vec<T> where T: Copy {
        match &self.root {
            Some(node) => unsafe { 
                let mut values: Vec<T> = Vec::new();
                node.as_ref().unwrap().inorder_rec(&mut values);
                return values;
            },
            None => Vec::new()
        }
    }

    pub fn exists_rec(&self, value: T) -> bool {
        match self.root {
            Some(node) => unsafe { (*node).exists(value) },
            None => false
        }
    }

    

    pub fn print_tree_rec(&self) where T: Debug {
        match self.root {
            Some(node) =>  unsafe { node.as_ref().unwrap().print_tree_recursive(0) },
            None => println!("Empty Tree")
        }
    }
}

impl<T> Datastructure<T> for BinaryTree<T> where T: PartialOrd + PartialEq{
    fn exists(&self, value: T) -> bool {
        let mut found = false;

        // Empty Tree
        if self.root.is_none() {
            return false;
        }

        let mut aux = self.root;

        while let Some(current) = aux {
            unsafe {
                if (*current).value == value {
                    found = true;
                    break;
                } else if value < (*current).value {
                    aux = (*current).left;
                } else {
                    aux = (*current).right;
                }
            }
        }

        return found;
    }

    // TODO: Decrement the size if the element is removed.
    // TODO: Refactor this function, too large.
    fn remove(&mut self, value: T) {
        let mut aux = self.root;
        let mut prev_node: NodeRef<T> = None;

        while let Some(current) = aux {
            unsafe {
                // Left tree
                if value < (*current).value {
                    aux = (*current).left;
                    prev_node = Some(current);

                // Right tree
                } else if value > (*current).value {
                    aux = (*current).right;
                    prev_node = Some(current);
                    
                // Node found
                } else {
                    let prev_node_aux = prev_node.unwrap();

                    // Is leaft
                    if (*current).is_leaft() {
                        // Delete root
                        if prev_node.is_none() {
                        
                        // Delete node
                        } else { 
                            // Left child
                            if value < (*prev_node_aux).value {
                                (*prev_node_aux).left = None;
                                // right child
                            } else {
                                (*prev_node_aux).right = None;
                            }
                            // Use box to turns raw pointer back to a box and free the memory
                            drop(Box::from_raw(current));
                        }

                    // Has one child
                    } else if (*current).has_one_child() {
                        // Set current node child to prev_node
                        // Left child
                        if (*current).left.is_some() {
                            if value < (*prev_node_aux).value {
                                (*prev_node_aux).left = (*current).left;
                            } else {
                                (*prev_node_aux).right = (*current).left;
                            }

                        // Right child
                        } else {
                            if value < (*prev_node_aux).value {
                                (*prev_node_aux).left = (*current).right;
                            } else {
                                (*prev_node_aux).right = (*current).right;
                            }
                        }
                        // Deallocate memory from removed node
                        drop(Box::from_raw(current));

                    // Has tow childs
                    } else {
                        // Find the littlest node in the right subtree (successor)
                        let successor= self.find_smallest((*current).right.unwrap());

                        // // Swap node to detete with the successor

                        // This operation will move the left subtree of the current node to the successor left subtree
                        // This means:
                        // successor.left is stored in an example address: 0x01    
                        // current.left is stored in an example address: 0x02
                        // before the equal assigment, current will move to successor:
                        // successor.left = 0x02    
                        // current.left = 0x02 
                        // At the end of the function, the current pointer is deallocated, so successor too.   
                        // (*successor).left = (*current).left

                        // This is the way to solve this problem, ptr::read return the value form the pointer without moving it.
                        let node = ptr::read(current);


                        // Set successor left child to current left child
                        (*successor).left = node.left;


                        // Swap current (node to delete) with successor
                        if (*(*prev_node_aux).left.unwrap()).value == value {
                            (*prev_node_aux).left = Some(successor);
                        } else {
                            (*prev_node_aux).right = Some(successor);
                        }

                        // Deallocate memory of the current node which is deleted from the tree.
                        drop(Box::from_raw(current));
                    }
                    
                    break;
                }
            }
        }
    }

    fn insert(&mut self, value: T) {
        if self.root.is_none() { 
            self.root = Some(Node::new(value)) 
        } else {
            let mut current_node = self.root.unwrap();

            loop {
                unsafe {
                    if value < (*current_node).value {
                        if (*current_node).left.is_none() {
                            (*current_node).left = Some(Node::new(value));
                            break;
                        } else {
                            current_node = (*current_node).left.unwrap();
                        }
                    } else if value > (*current_node).value {
                        if (*current_node).right.is_none() {
                            (*current_node).right = Some(Node::new(value));
                            break;
                        } else {
                            current_node = (*current_node).right.unwrap();
                        }
                    // Value already in tree, go out to the loop and finish the insertion    
                    } else { 
                        break;
                    }
                }
            }
        }

        self.size += 1;
    }
}