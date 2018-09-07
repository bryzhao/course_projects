// Bryan Zhao
// ECS 40, Fall 2017
// Homework 5

// tree definition
#[derive(Debug)]
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
        		if key == &(*cur_node) {
        			return true;
        		} else if key < &(*cur_node) {
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
    pub fn preorder(&self) -> IterPreorder<T> {
			IterPreorder::new(Some(Box::new(self)))
    }

     /// returns the inorder traversal of the tree.
    pub fn inorder(&self) -> IterInorder<T> {
			IterInorder::new(Some(Box::new(self)))
    }

    /// returns the postorder traversal of the tree.
    pub fn postorder(&self) -> IterPostorder<T> {
    	IterPostorder::new(Some(Box::new(self)))
    }
}

pub struct IterPreorder<'a, T: 'a> {
    stack: Vec<&'a Tree<T>>, // bind
}

impl<'a, T> IterPreorder<'a, T> {
		pub fn new(tree: Option<Box<&'a Tree<T>>>) -> IterPreorder<T> {
			let mut pre_out = IterPreorder { stack: Vec::new() }; // init
			tree.as_ref().map(|tree| {
				pre_out.stack.push(tree); // push root of tree to s
			});
			pre_out // return output
		}
}

impl<'a, T> Iterator for IterPreorder<'a, T> {
	type Item = &'a T; // type parameter

	fn next(&mut self) -> Option<Self::Item> {
		// push popped elements.right, then .left to s, if they exist
		let popped_var = self.stack.pop();
		match popped_var { 
			Some(ref x) => {
				match x.right {
					Some(ref right) => {
						self.stack.push(&*right);
					},
					None => {},
				}
				match x.left {
					Some(ref left) => {
						self.stack.push(&*left);
					},
					None => {},
				}
				match x.data {
					Some (ref val) => {
						return Some(&(*val))
					},
					None => { return None },
				}
			},
			None => { return None },
		} // end match
	}  
}

pub struct IterInorder<'a, T: 'a> {
    stack: Vec<&'a Tree<T>>, // bind
}

impl<'a, T> IterInorder<'a, T> {
		pub fn new(tree: Option<Box<&'a Tree<T>>>) -> IterInorder<T> {
			let mut in_out = IterInorder { stack: Vec::new() }; // init
			tree.as_ref().map(|tree| {
				in_out.stack.push(tree); // root
			});
			in_out // return output, as root
		}
}

impl<'a, T> Iterator for IterInorder<'a, T> {
	type Item = &'a T; // type parameter

	fn next(&mut self) -> Option<Self::Item> {

		// track current variable
		let mut current_tree = self.stack.pop(); // set current = root

		match current_tree { 
			Some(ref cur_tree) => {
				self.stack.push(&*cur_tree); // if current not None, push current to stack
				current_tree = cur_tree.left // set current = current.left
			},
			None => {
				if self.stack.len() >= 1 {
					current_tree = self.stack.pop();
					match current_tree.data {
						Some (ref val) => return Some(&(*val)),
						None => return None,
					}
					current_tree = current_tree.right; // set current = current.right
				} else { // if current is None and stack is not empty, pop from stack (print value)
					return None // do nothing
				}
			},
		} // end match current_tree


			/* => {
				match x.right {
					Some(ref right) => {
						self.stack.push(&*right);
					},
					None => {},
				}
				match x.left {
					Some(ref left) => {
						self.stack.push(&*left);
					},
					None => {},
				}
				match x.data {
					Some (ref val) => {
						return Some(&(*val))
					},
					None => { return None },
				}
			},
			None => { return None },
		} // end match */
	} 
}

pub struct IterPostorder<'a, T: 'a> {
    stack: Vec<&'a Tree<T>>, // bind
}

impl<'a, T> IterPostorder<'a, T> {
		pub fn new(tree: Option<Box<&'a Tree<T>>>) -> IterPostorder<T> {
			let mut post_out = IterPostorder { stack: Vec::new() }; // init
			tree.as_ref().map(|tree| {
				post_out.stack.push(tree);
			});			
			post_out
		}
}

impl<'a, T> Iterator for IterPostorder<'a, T> {
	type Item = &'a T; // type parameter

	fn next(& mut self) -> Option<Self::Item> {
		// push popped elements.right, then .left to s, if they exist
		let popped_var = self.stack.pop();
		match popped_var { 
			Some(ref x) => {
				match x.right {
					Some(ref right) => {
						self.stack.push(&*right);
					},
					None => {},
				}
				match x.left {
					Some(ref left) => {
						self.stack.push(&*left);
					},
					None => {},
				}
				match x.data {
					Some (ref val) => {
						return Some(&(*val))
					},
					None => { return None },
				}
			},
			None => { return None },
		} // end match
	}   
}

#[test]
	fn it_works() {
	let mut root = Tree::<i32>::new();
	let data = vec![3, 2, 1, 5, 4, 6, 7];
	for v in data {
	    root.insert(v);
	}

	println!("Preorder");
	for v in root.preorder() {
	    println!("{:?}", v);
	};

  println!("Inorder");
  for v in root.inorder() {
      println!("{:?}", v);
  };

  println!("Postorder");
  for v in root.postorder() {
      println!("{:?}", v);
  };
} 
