#[derive(Debug)]
pub struct LinkedList<T> ( // using () means that the fiels are pub by default, instead of {}
    Option<(T,Box<LinkedList<T>>)>
);

//alternative code for LL struct:
// pub struct LinkedList<T> {
//     pub node: Option<(T, Box<LinkedList<T>>)>
// }

impl<T:PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t=self.0.take();
        self.0 = Some((data,Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_,ref mut child)) => child.push_back(data),
            None => self.push_front(data),            
        }
    }

    /*
    pub fn insert_sorted(&mut self, data: T) {
        match self.0 {
            Some((cur_val,ref mut child)) => {
                if cur_val < data {
                    match child.0 {
                        Some((next_val,_)) => {
                            if next_val > data {
                                child.insert_sorted(data)
                            }
                            else {
                                let t=self.0.take();
                                self.0 = Some((data,Box::new(LinkedList(t))));
                            }
                        },
                        None => self.push_front(data),                        
                    }

                }
                
            },
            None => self.push_front(data),            
        }
    }
    */
}