use std::io;

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Build the binary tree
    let mut root = TreeNode::new(1);
    let mut left_child = TreeNode::new(2);
    let right_child = TreeNode::new(3);

    let left_left_child = TreeNode::new(4);
    let left_right_child = TreeNode::new(5);

    left_child.left = Some(Box::new(left_left_child));
    left_child.right = Some(Box::new(left_right_child));

    root.left = Some(Box::new(left_child));
    root.right = Some(Box::new(right_child));

    // Calculate the maximum depth of the binary tree
    let depth = max_depth(&Some(Box::new(root)));
    println!("Maximum depth of the binary tree is: {}", depth);
}
