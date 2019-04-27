use std::cmp::{Ord, Ordering};

pub struct Tree<T> {
	val:Option<T>,
	left:Option<Box<Tree<T>>>,
	right:Option<Box<Tree<T>>>,
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Tree::<T>{
			val:None,
			left:None,
			right:None,
		}
    }
    
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

    pub fn preorder(&self) -> IterPreorder<T> {
        IterPreorder::<T>::new(self)
    }

    pub fn inorder(&self) -> IterInorder<T> {
        IterInorder::<T>::new(self)
    }

    pub fn postorder(&self) -> IterPostorder<T> {
        IterPostorder::<T>::new(self)
    }
}

pub struct IterPreorder<'a, T: 'a> {
    stack:Vec<&'a Tree<T>>,
}

impl<'a, T> IterPreorder<'a, T> {
    fn new(root:&'a Tree<T>) -> Self {
    	IterPreorder{
    		stack:vec![root],
    	}
    }
}

impl<'a, T> Iterator for IterPreorder<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
    	while self.stack.len() > 0 {
    		let now = self.stack.pop().unwrap();
    		if let Some(ref x) = now.right {
    			self.stack.push(x.as_ref());
    		}
    		if let Some(ref x) = now.left {
    			self.stack.push(x.as_ref());
    		}
    		if let Some(ref x) = now.val {
    			return Some(x);
    		}
    	}
    	return None;
    }
}

pub struct IterInorder<'a, T: 'a> {
    stack:Vec<(&'a Tree<T>,i32)>,
}

impl<'a, T> IterInorder<'a, T> {
	fn new(root:&'a Tree<T>) -> Self {
		IterInorder{
			stack:vec![(root,0)],
		}
    }
}

impl<'a, T> Iterator for IterInorder<'a, T> {
	type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
    	while self.stack.len() > 0 {
    		let (x,y) = self.stack.pop().unwrap();
    		if y == 1 {
    			if let Some(ref c) = x.val {
    				return Some(c);
    			}
    		}
    		if let Some(ref right) = x.right {
    			self.stack.push((right,0));
    		}
    		self.stack.push((x,1));
    		if let Some(ref left) = x.left {
    			self.stack.push((left,0));
    		}
    	}
    	return None;
   	}
}

pub struct IterPostorder<'a, T: 'a> {
    stack:Vec<(&'a Tree<T>,i32)>,
}

impl<'a, T> IterPostorder<'a, T> {
    fn new(root:&'a Tree<T>) -> Self {
		IterPostorder{
			stack:vec![(root,0)],
		}
    }
}

impl<'a, T> Iterator for IterPostorder<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		while self.stack.len() > 0 {
    		let (x,y) = self.stack.pop().unwrap();
    		if y == 1 {
    			if let Some(ref c) = x.val {
    				return Some(c);
    			}
    		}
    		self.stack.push((x,1));
    		if let Some(ref right) = x.right {
    			self.stack.push((right,0));
    		}
    		if let Some(ref left) = x.left {
    			self.stack.push((left,0));
    		}
    	}
    	return None;
	}
}

fn main() {
    let mut root = Tree::<i32>::new();
    let data = vec![3, 2, 1, 5, 4, 6, 7];
    for v in data {
        root.insert(v);
    }

    println!("Preorder");
    for v in root.preorder() {
        println!("{}", v);
    };

    println!("Inorder");
    for v in root.inorder() {
        println!("{}", v);
    };

    println!("Postorder");
    for v in root.postorder() {
        println!("{}", v);
    };
}