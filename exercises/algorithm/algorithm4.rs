/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

/*
impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}
*/

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + std::fmt::Debug
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + std::fmt::Debug
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + std::fmt::Debug
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert_sub(root: &mut Option<Box<TreeNode<T>>>, value: T) {
        if let None = root {*root = Some(Box::new(TreeNode::new(value))); return;}
        let node: &mut Box<TreeNode<T>> = root.as_mut().unwrap();
        if node.value == value {return;}
        if value < node.value {Self::insert_sub(&mut node.left, value); return;}
        Self::insert_sub(&mut node.right, value);
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        Self::insert_sub(&mut self.root, value)
        //; dbg!(&self);
    }

    fn search_sub(root: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        if let None = root {return false;}
        let node = root.as_ref().unwrap();
        if *value == node.value {return true;}
        if *value < node.value {return Self::search_sub(&node.left, value);}
        return Self::search_sub(&node.right, value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        Self::search_sub(&self.root, &value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


