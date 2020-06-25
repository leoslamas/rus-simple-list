#[derive(Debug, PartialEq)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }
    pub fn add(&mut self, item: T) {
        let mut last = &mut self.head;

        while let Some(node) = last {
            last = &mut node.next;
        }

        let node = Box::new(Node {
            item: item,
            next: None,
        });

        *last = Some(node);
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        let mut tmp = &self.head;
        let mut counter = 0;

        while let Some(node) = tmp {
            if counter == index {
                return Some(&node.item);
            }
            tmp = &node.next;
            counter += 1;
        }

        None
    }

    pub fn del(&mut self, _index: u32) {
        unimplemented!()
    }
    
    pub fn pop(&mut self) -> Option<T> {
        let mut result: Option<T> = None;
        
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            result = Some(head.item);
        });
        
        result
    }

    pub fn size(&self) -> u32 {
        let mut tmp = &self.head;
        let mut size = 0;

        while let Some(node) = tmp {
            tmp = &node.next;
            size += 1;
        }

        size
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> LinkedList<i32> {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(10);
        list.add(100);
        list
    }

    #[test]
    fn test_list_new() {
        let actual = LinkedList::<i32>::new();

        let expect = LinkedList::<i32> {
            head: None,
        };

        assert_eq!(expect, actual);
    }

    #[test]
    fn test_list_has_right_size() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(10);
        list.add(100);

        assert_eq!(3, list.size());
    }

    #[test]
    fn test_list_should_add_element() {
        let list = setup();

        let e1 = list.get(0);
        let e2 = list.get(1);
        let e3 = list.get(2);
        
        assert_eq!(Some(&1), e1);
        assert_eq!(Some(&10), e2);
        assert_eq!(Some(&100), e3);
    }

    #[test]
    fn test_list_should_get_right_index() {
        let list = setup();

        let e1 = list.get(0);
        let e2 = list.get(1);
        let e3 = list.get(2);

        assert_eq!(Some(&1), e1);
        assert_eq!(Some(&10), e2);
        assert_eq!(Some(&100), e3);
        
        let e4 = list.get(3);

        assert_eq!(None, e4);
    }

    #[test]
    fn test_should_iterate_over_list() {
        let list = setup();

        let mut it = list.iter();
        assert_eq!(Some(&1), it.next());
        assert_eq!(Some(&10), it.next());
        assert_eq!(Some(&100), it.next());
    }

    #[test]
    fn test_should_pop_first_element() {
        let mut list = setup();

        let e1 = list.get(0);
        let e2 = list.get(1);
        let e3 = list.get(2);

        assert_eq!(Some(&1), e1);
        assert_eq!(Some(&10), e2);
        assert_eq!(Some(&100), e3);

        let e4 = list.pop();

        assert_eq!(2, list.size());
        assert_eq!(Some(1), e4);

        let e4 = list.get(0);
        let e5 = list.get(1);

        assert_eq!(Some(&10), e4);
        assert_eq!(Some(&100), e5);
    }

    #[test]
    #[ignore]
    fn test_should_del_element_by_index() {
        let mut list = setup();

        let e1 = list.get(0);
        let e2 = list.get(1);
        let e3 = list.get(2);

        assert_eq!(Some(&1), e1);
        assert_eq!(Some(&100), e3);
        assert_eq!(Some(&10), e2);

        list.del(1);

        let e4 = list.get(0);
        let e5 = list.get(1);
        let e6 = list.get(2);

        assert_eq!(Some(&1), e4);
        assert_eq!(Some(&100), e5);
        assert_eq!(None, e6);
    }
}
