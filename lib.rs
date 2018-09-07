// Bryan Zhao
// ECS 40, Fall 2017
// Homework 4

// tree definition
pub struct Tree<T> {
	data: Option<T>, 
	left: Option<Box<Tree<T>>>, // left child
	right: Option<Box<Tree<T>>>, // right child
}

impl<T: Ord> Tree<T> {
    /// creates an empty tree
    pub fn new() -> Self {
        Tree { // empty tree pointing to nothing
        	data: None, 
        	left: None, 
        	right: None,
        }
    }

    /// returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
    	match self.data {
    		Some(ref mut cur_node) => { // if a tree exists, pass current key

    			let node_ptr = { // set mutable reference, determine where to point
    			if key == *cur_node {
	    				return false;
	    			} else if key < *cur_node {
	    				&mut self.left
	    			} else {
	    				&mut self.right
	    			}
    			};

    			match node_ptr { // point to next child, but if none, insert node
    				&mut Some(ref mut node) => node.insert(key), // keep going down tree
    				&mut None => { // if no child, insert!
    					let new_node = Tree {
    						data: Some(key),
    						left: None,
    						right: None,
    					};
    					*node_ptr = Some(Box::new(new_node)); // point to the new node
    					return true
    				} // create new node
    			}
    		},

    		None => { // if no root, insert node
    			self.data = Some(key);
    			return true
    		},
    	} // end of outer match
    }

    /// search, returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.data {
        	Some(ref cur_node) => {
        		if key == cur_node {
        			return true;
        		} else if key < cur_node {
        				match self.left {
        					Some(ref node) => node.find(key),
        					None => return false, // not in the left branch!
        				}
        			} else {
        					match self.right {
        						Some(ref node) => node.find(key),
        						None => return false, // not in the right branch!
        					}
        				}
        			}, // end of Some block
        	None => return false, // not in the root of the tree
        }
    } // end fn!

    /// returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut pre_vec = Vec::<&T>::new(); // preorder output

        match self.data {
        	Some (ref cur_node) => {
        		pre_vec.push(cur_node); // display data of current root/node
        		match self.left { // traverse left subtree, recursively call preorder
        			Some (ref left_node) => {
        				pre_vec.extend(left_node.preorder());
        			},
        			None => {},
        		};
        		match self.right { // traverse right substree, recursively call preorder
        			Some (ref right_node) => {
        				pre_vec.extend(right_node.preorder());
        			},
        			None => {},
        		};
        	},
					None => {},  // check if current node is empty
        } 
        pre_vec // return output
    }

    /// returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut in_vec = Vec::<&T>::new(); // in-order output

        match self.data {
        	Some (ref cur_node) => {
        		match self.left {
        			Some (ref left_node) => {
        				in_vec.extend(left_node.inorder());
        			},
        			None => {}, // do nothing
        		} // traverse left subtree recursively
        		in_vec.push(cur_node); // now, display data of current root/node
        		match self.right {
        			Some (ref right_node) => {
        				in_vec.extend(right_node.inorder());
        			},
        			None => {}, // do nothing
        		}
        	},
        	None => {}, // check if current node empty
        }
        in_vec // return output
    }

    /// returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut post_vec = Vec::<&T>::new(); // post-order output

        match self.data {
        	Some(ref cur_node) => {
        		match self.left { // traverse left subtree recursively
        			Some (ref left_node) => {
        				post_vec.extend(left_node.postorder());
        			},
        			None => {}, // do nothing
        		}
        		match self.right { // traverse right subtree recursively
        			Some (ref right_node) => {
        				post_vec.extend(right_node.postorder());
        			},
        			None => {}, // do nothing
        		}
        		post_vec.push(cur_node); // and then, display data of current node!
        	}, // do something
        	None => {}, // do nothing
        }

        post_vec // return output
    }
}

// run test cases
#[test]
fn it_works() {
	let mut tree = Tree::<i32>::new();
	tree.insert(4);
	tree.insert(5);
	tree.insert(3);
	tree.insert(3);
	assert_eq!(tree.find(&6), false); // test for equivalence!
}

#[test]
fn preorder() {
	let mut b_tree: Tree<i32> = Tree::new();
	b_tree.insert(3);
	b_tree.insert(2);
	b_tree.insert(4);
	b_tree.insert(1);
	let pre_ordered = b_tree.preorder();
	let a = 3; let b = 2; let c = 1; let d = 4;
	let mut ordered: Vec<&i32> = vec![&a,&b,&c,&d];
	assert_eq!(pre_ordered, ordered);
}

#[test]
fn inorder() {
	let mut btree: Tree<i32> = Tree::new(); // empty tree
	btree.insert(12); btree.insert(14); btree.insert(16); 
	btree.insert(8); btree.insert(2); btree.insert(13); 
	let in_order = btree.inorder();
	let a = 2; let b = 8; let c = 12;
	let d = 13; let e = 14; let f = 16;
	let mut ordered: Vec<&i32> = vec![&a, &b, &c, &d, &e, &f];
	assert_eq!(in_order, ordered);
}

#[test]
fn postorder() {
	let mut btree: Tree<i32> = Tree::new(); // empty tree
	btree.insert(12); btree.insert(14); btree.insert(16); 
	btree.insert(8); btree.insert(2); btree.insert(13); 
	let post_order = btree.postorder();
	let a = 2; let b = 8; let c = 13;
	let d = 16; let e = 14; let f = 12;
	let mut ordered: Vec<&i32> = vec![&a, &b, &c, &d, &e, &f];
	assert_eq!(post_order, ordered);
}
