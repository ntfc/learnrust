#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug,PartialEq)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Debug)]
pub struct IntoIter<T>(BST<T>);

#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        return self.root.insert(value)
    }

    pub fn search(&self, value: T) -> bool {
        return self.root.search(value)
   }
}

trait InsertSearch<T: Ord> {
    fn insert(&mut self, value: T) -> bool;
    fn search(&self, value: T) -> bool;

}

impl<T: Ord> InsertSearch<T> for Link<T> {
    fn insert(&mut self, value: T) -> bool {
        match *self {
            None => {
                let new_node = Box::new(Node {
                    elem: value,
                    left: None,
                    right: None
                });
                *self = Some(new_node);
                true
            }
            Some(ref mut boxed_node) => {
                if boxed_node.elem > value {
                    // recurse left
                    boxed_node.left.insert(value)
                }
                else if boxed_node.elem < value {
                    // recurse right
                    boxed_node.right.insert(value)
                }
                else {
                    false
                }
            }
        }
    }

    fn search(&self, value: T) -> bool {
        match *self {
            None => false,
            Some(ref boxed_node) => {
                if boxed_node.elem > value {
                    // recurse left
                    boxed_node.left.search(value)
                }
                else if boxed_node.elem < value {
                    // recurse right
                    boxed_node.right.search(value)
                }
                else {
                    true
                }
            }
        }
    }
}

// implementation of the iterator
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.root.take().map(|boxed_node| {
            let node = *boxed_node;
            self.0.root = node.right;
            node.elem
        })
    }
}

// implementation of the iterator
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.right.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// implementation of the iterator
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

// sugar use iterator on for loops
impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

// sugar to use iterator on for loops
impl <'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { next: self.root.as_ref().map(|node| &**node) }
    }
}

// sugar to use iterator on for loops
impl <'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut { next: self.root.as_mut().map(|node| &mut **node) }
    }
}

#[cfg(test)]
mod test_bst {
    use super::BST;
    use super::Node;

    #[test]
    fn bst_insert() {
        let mut bst = BST::new();
        assert_eq!(bst.insert(20), true);
        assert_eq!(bst.root.as_ref(), Some(&Box::new(Node { elem: 20, left: None, right: None })));
        assert_eq!(bst.insert(10), true);
        assert_eq!(bst.insert(30), true);
        assert_eq!(bst.insert(30), false);
        assert_eq!(bst.insert(5), true);
        assert_eq!(bst.insert(40), true);
    }

    #[test]
    fn bst_search() {
        let mut bst = BST::new();
        assert_eq!(bst.search(20), false);
        assert_eq!(bst.insert(20), true);
        assert_eq!(bst.search(20), true);

        assert_eq!(bst.insert(10), true);
        assert_eq!(bst.search(30), false);
        assert_eq!(bst.insert(30), true);
        assert_eq!(bst.insert(5), true);
        assert_eq!(bst.insert(40), true);
        assert_eq!(bst.search(30), true);
    }
}

#[cfg(test)]
mod test_link {
    use super::InsertSearch;

    #[test]
    fn insert_test() {
        let mut link = None;
        assert_eq!(link.insert(2), true);
        assert_eq!(link.insert(2), false);
        assert_eq!(link.insert(3), true);
        assert_eq!(link.insert(3), false);
        assert_eq!(link.insert(1), true);
        assert_eq!(link.insert(5), true);
    }

    #[test]
    fn search_test() {
        let mut link = None;
        assert_eq!(link.search(2), false);
        assert_eq!(link.insert(5), true);
        assert_eq!(link.insert(3), true);
        assert_eq!(link.insert(10), true);
        assert_eq!(link.search(10), true);
        assert_eq!(link.search(6), false);
    }
}

#[cfg(test)]
mod test_iter {
    use super::BST;

    #[test]
    fn into_iter_compiles() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(4);
        bst.insert(3);
        for elem in bst {
            println!("{}", elem);
        }
    }

    #[test]
    fn into_iter() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(4);
        bst.insert(3);
        let mut iter = bst.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ref_into_iter_compiles() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(4);
        bst.insert(3);
        println!("ref iter");
        for elem in &bst {
            println!("{}", elem);
        }
    }

    #[test]
    fn ref_into_iter() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(4);
        bst.insert(3);
        let mut iter = (&bst).into_iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mut_into_iter_compiles() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(4);
        bst.insert(3);
        println!("ref iter");
        for elem in &mut bst {
            println!("{}", elem);
        }
    }

    #[test]
    fn mut_into_iter() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(4);
        bst.insert(3);
        let mut iter = (&mut bst).into_iter();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
