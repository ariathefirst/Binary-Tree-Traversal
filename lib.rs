pub struct Tree<T> {
	root: Option<Box<TreeNode<T>>>, // can be some or none(null)
}
pub struct TreeNode<T> {
	key: T,
	l: Option<Box<TreeNode<T>>>,
	r: Option<Box<TreeNode<T>>>,
}
use std::cmp::Ord;
impl<T: Ord> TreeNode<T> {
    pub fn insert(&mut self, key: T) -> bool {
        if self.key == key {
            return false;
        }
        let insert_node = if key < self.key {
                            &mut self.l 
                          } else { 
                            &mut self.r 
                          };
        match insert_node {
            &mut Some(ref mut subnode) => subnode.insert(key),
            &mut None => {
                // let new_node = TreeNode {
                //     key: key,
                //     l: None,                                    
                //     r: None,
                // };
                // let boxed_node = Some(Box::new(new_node));
                // *insert_node = boxed_node;
                *insert_node = Some(Box::new(TreeNode{
                    key: key,
                    l: None,
                    r: None,
                }));
                return true;
            },
        }
    }
    pub fn find(&self, key: &T) -> bool {
        if self.key == *key {
            return true;
        }
        let find_node = if self.key > *key {
            &self.l
        } else {
            &self.r
        };
        match find_node {
            &Some(ref subnode) => {
                subnode.find(key)
            },
            &None => return false,
        }               
    }
    pub fn pre<'a>(&'a self, vec: &mut Vec<&'a T>) {
        vec.push(&self.key);
        match self.l {
            Some(ref l) => l.pre(vec),
            None => {},
        }
        match self.r {
            Some(ref r) => r.pre(vec),
            None => {},
        }
    }
    pub fn ino<'a>(&'a self, vec: &mut Vec<&'a T>) {
        match self.l {
            Some(ref l) => l.ino(vec),
            None => {},
        }
        vec.push(&self.key);
        match self.r {
            Some(ref r) => r.ino(vec),
            None => {},
        }
    }
    pub fn post<'a>(&'a self, vec: &mut Vec<&'a T>) {
        match self.l {
            Some(ref l) => l.post(vec),
            None => {},
        }
        match self.r {
            Some(ref r) => r.post(vec),
            None => {},
        }
        vec.push(&self.key);
    }
}
impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
		Tree {
			root: None,
		}
	}
// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    //fn recurse(TreeNode
    pub fn insert(&mut self, key: T) -> bool {
        match self.root {
            Some(ref mut r) => r.insert(key),
            None => {
                // let root_node = TreeNode {
                //     key: key,
                //     l: None,
                //     r: None,
                // };
                // let boxed_node = Some(Box::new(root_node));
                // self.root = boxed_node;
                self.root = Some(Box::new(TreeNode{
                    key: key,
                    l: None,
                    r: None,
                    }));
                return true;
            },
        }    
	}
	/// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.root {
            Some(ref r) => r.find(key),
            None => return false,
        }
    }
        /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        // let stack: Vec<Box<TreeNode<T>>> = Vec::new();
        // let res: Vec<&T> = Vec::new();
        // match self.root {
        //     Some(ref r) => stack.push(*r),
        //     None => {},
        // }
        // while !stack.is_empty() {
        //     match stack.pop() {
        //         Some(ref s) => { 
        //             //let cur = s; 
        //             match s.l {
        //                 Some(ref l) => { 
        //                     stack.push(*l);
        //                     res.push(&s.key);
        //                 },
        //                 None => {}
        //             }
        //             match s.r {
        //                 Some(ref r) => {
        //                     stack.push(*s);
        //                     res.push(&s.key);
        //                 },
        //                 None => {}
        //             }                    
        //         },
        //         None => {},
        //     }
        //     //let cur = stack.pop();
        //     // match cur.l {
        //     //     Some(ref l) => stack.push(l),
        //     //     None => {}
        //     // }
        //     // match cur.r {
        //     //     Some(ref r) => stack.push(r),
        //     //     None => {}
        //     // }
        //     //res.push(cur.key);
        // }
        let mut vec: Vec<&T> = Vec::new();
        match self.root {
            Some(ref r) => r.pre(&mut vec),
            None => {},
        }
        vec
    }
    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut vec: Vec<&T> = Vec::new();
        match self.root {
            Some(ref r) => r.ino(&mut vec),
            None => {},
        }
        vec
    }
    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut vec: Vec<&T> = Vec::new();
        match self.root {
            Some(ref r) => r.post(&mut vec),
            None => {},
        }
        vec
    }
}
