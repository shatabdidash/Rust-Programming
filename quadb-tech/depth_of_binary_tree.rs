// Define the binary tree node structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Constructor to create a new TreeNode
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Function to calculate the maximum depth of the binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example binary tree
    let mut root = TreeNode::new(3);
    let mut left_child = TreeNode::new(9);
    let mut right_child = TreeNode::new(20);
    let mut right_left_child = TreeNode::new(15);
    let right_right_child = TreeNode::new(7);

    right_child.left = Some(Box::new(right_left_child));
    right_child.right = Some(Box::new(right_right_child));

    root.left = Some(Box::new(left_child));
    root.right = Some(Box::new(right_child));

    // Calculate and print the maximum depth of the binary tree
    println!("Maximum depth of the binary tree: {}", max_depth(Some(Box::new(root))));
}
