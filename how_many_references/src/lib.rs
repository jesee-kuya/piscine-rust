pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|rc| !Rc::ptr_eq(rc, &element));
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));
        let a1 = Rc::new(String::from("a"));
        let mut new_node = Node::new(vec![a.clone()]);
        new_node.add_element(b.clone());
        new_node.add_element(a.clone());
        new_node.add_element(c.clone());
        new_node.add_element(a.clone());
        assert_eq!(how_many_references(&a), 4);
        assert_eq!(how_many_references(&b), 2);
        assert_eq!(how_many_references(&c), 2);
        new_node.rm_all_ref(a1.clone());
        new_node.rm_all_ref(a.clone());
        assert_eq!(how_many_references(&a), 1);
        assert_eq!(how_many_references(&b), 2);
        assert_eq!(how_many_references(&c), 2);
    }
}
