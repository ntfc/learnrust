#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl BST {
    pub fn new() -> Self {
        BST { root: Link::Empty }
    }

    pub fn insert(&mut self, value: i32) -> bool {
        return self.root.insert(value)
    }

    pub fn search(&self, value: i32) -> bool {
        return self.root.search(value)
   }
}

impl Link {
    pub fn insert(&mut self, value: i32) -> bool {
        match *self {
            Link::Empty => {
                let new_node = Box::new(Node {
                    elem: value,
                    left: Link::Empty,
                    right: Link::Empty
                });
                *self = Link::More(new_node);
                true
            }
            Link::More(ref mut boxed_node) => {
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

    pub fn search(&self, value: i32) -> bool {
        match *self {
            Link::Empty => false,
            Link::More(ref boxed_node) => {
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


#[cfg(test)]
mod test_bst {
    use super::BST;

    #[test]
    fn bst_insert() {
        let mut bst = BST::new();
        assert_eq!(bst.insert(20), true);
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
    use super::Link;

    #[test]
    fn insert_test() {
        let mut link = Link::Empty;
        assert_eq!(link.insert(2), true);
        assert_eq!(link.insert(2), false);
        assert_eq!(link.insert(3), true);
        assert_eq!(link.insert(3), false);
        assert_eq!(link.insert(1), true);
        assert_eq!(link.insert(5), true);
    }

    #[test]
    fn search_test() {
        let mut link = Link::Empty;
        assert_eq!(link.search(2), false);
        assert_eq!(link.insert(5), true);
        assert_eq!(link.insert(3), true);
        assert_eq!(link.insert(10), true);
        assert_eq!(link.search(10), true);
        assert_eq!(link.search(6), false);
    }
}
