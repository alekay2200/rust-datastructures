use rust_datastructures::{Datastructure, BinaryTree};

fn main() {
    let mut tree = BinaryTree::new();

    for n in [50,17,72,12,23,54,76,9,14,19,67,15] {
        tree.insert(n);
    }

    println!("\n------------------------------------------\n");

    tree.print_tree_rec();    

    println!("\n------------------------------------------\n");

    let mut n = 12;
    println!("Exists [{n}] in tree: {}", tree.exists_rec(n));

    tree.remove(n);
    println!("Removing element {n}");

    println!("Exists [{n}] in tree: {}", tree.exists_rec(n));

    println!("\n------------------------------------------\n");

    tree.print_tree_rec();    

    println!("\n------------------------------------------\n");

    // let values = tree.preorder();
    // println!("\n{:?}\n", values);
    
    println!("\n------------------------------------------\n");

    println!("Inorder: {:?}", tree.inorder());

    println!("\n------------------------------------------\n");

    n = 3;
    println!("Exists [{n}] in tree: {}\n\n", tree.exists(n));
    n = 100;
    println!("Exists [{n}] in tree: {}\n\n", tree.exists(n));
    n = 67;
    println!("Exists [{n}] in tree: {}\n\n", tree.exists(n));
}