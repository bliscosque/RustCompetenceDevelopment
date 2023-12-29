use std::cell::RefCell;
use std::rc::{Rc,Weak};

#[derive(Debug)]
pub struct DLL<T> {
    first: Option<Rc<RefCell<DNode<T>>>>,
    last: Option<Weak<RefCell<DNode<T>>>>, //is deleted even if have weak pointer
}

#[derive(Debug)]
pub struct DNode<T> {
    data: T,
    next: Option<Rc<RefCell<DNode<T>>>>,
    prev: Option<Weak<RefCell<DNode<T>>>>, //is deleted even if have weak pointer
}

impl<T> DLL<T> {
    pub fn new()->Self {
        DLL {
            first:None,
            last:None,
        }
    }
    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                let new_front=Rc::new(RefCell::new(DNode{data,next:Some(r.clone()), prev: None,}));
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            },
            None => {
                let new_data=Rc::new(RefCell::new(DNode{data,next:None, prev: None,}));
                self.last=Some(Rc::downgrade(&new_data));
                self.first=Some(new_data);
            },
        }
    }
    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                let new_back=Rc::new(RefCell::new(DNode{data,prev:Some(r.clone()), next: None,}));
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
            },
            None => {
                let new_data=Rc::new(RefCell::new(DNode{data,next:None, prev: None,}));
                self.last=Some(Rc::downgrade(&new_data));
                self.first=Some(new_data);
            },
        }
    }
 }