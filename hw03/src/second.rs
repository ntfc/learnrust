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

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.root.take().map(|boxed_node| {
            let node = *boxed_node;
            self.0.root = node.right;
            node.elem
        })
    }
}

impl<T: Ord> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
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
}
