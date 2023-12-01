use std::cell::RefCell;
use std::rc::Rc;

type Link=Option<Rc<RefCell<Node>>>;

#[derive(Clone,Debug)]
struct  Node {
    value: String,
    next: Link, //pointer to next node
}

//node of List
impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next:None,
        }))
    }
}

//Single Linked List
#[derive(Debug)]
pub struct LinkedList {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn append(&mut self, value: String) {
        let new=Node::new(value);
        match self.tail.take() { // take remove last elem and returns it
            Some(old) => old.borrow_mut().next=Some(new.clone()),
            None => self.head=Some(new.clone())
        };
        self.length+=1;
        self.tail=Some(new);
    }
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next)=head.borrow_mut().next.take() {
                self.head=Some(next);
            } else {
                self.tail.take();
            } 
            self.length-=1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .value
        })
    }
}