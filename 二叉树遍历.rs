use std::cmp::Ordering;
#[derive(Debug)]
pub struct Tree<T> {
    val:Option<T>,
	left:Option<Box<Tree<T>>>,
	right:Option<Box<Tree<T>>>,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree::<T>{
			val:None,
			left:None,
			right:None,
		}
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
    	if let Some(ref x) = self.val {
    		match key.cmp(x) {
				Ordering::Less => {
					match self.left {
						Some(ref mut l) => l.insert(key),
						None => {
							self.left = Some(Box::new(Tree::<T>{val:Some(key),left:None,right:None}));
							return true;
						},
					}
				},
				Ordering::Greater => {
					match self.right {
						Some(ref mut r) => r.insert(key),
						None => {
							self.right = Some(Box::new(Tree::<T>{val:Some(key),left:None,right:None}));
							return true;
						},
					}
				},
				_ => return false,
			}
    	}
    	else {
    		self.val = Some(key);
    		return true;
    	}
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
    	if let Some(ref x) = self.val {
    		match key.cmp(&x) {
				Ordering::Less => match self.left {
					Some(ref t) => t.find(key),
					None => return false,
				},
				Ordering::Greater => match self.right {
					Some(ref t) => t.find(key),
					None => return false,
				},
				_  => return true,
			}
    	}
    	else {
    		return false;
    	}
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut ans:Vec<&T> = Vec::<&T>::new();
        if let Some(ref x) = self.val {
        	ans.push(&x);
        }
		match self.left {
			Some(ref t) => ans.extend(t.preorder()),
			None => {},
		}
		match self.right {
			Some(ref t) => ans.extend(t.preorder()),
			None => {},
		}
		return ans;
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut ans:Vec<&T> = Vec::<&T>::new();
		match self.left {
			Some(ref t) => ans.extend(t.inorder()),
			None => {},
		}
		if let Some(ref x) = self.val {
        	ans.push(&x);
        }
		match self.right {
			Some(ref t) => ans.extend(t.inorder()),
			None => {},
		}
		return ans;
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut ans:Vec<&T> = Vec::<&T>::new();
		match self.left {
			Some(ref t) => ans.extend(t.postorder()),
			None => {},
		}
		match self.right {
			Some(ref t) => ans.extend(t.postorder()),
			None => {},
		}
		if let Some(ref x) = self.val {
        	ans.push(&x);
        }
		return ans;
    }
}
