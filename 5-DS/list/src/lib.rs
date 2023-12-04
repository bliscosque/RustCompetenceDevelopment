mod linked_list;
mod linked_list_box;

mod tests {
    use crate::*;
    #[test]
    fn linked_list() {
        //create
        let mut ll=linked_list::LinkedList::new();
        assert_eq!(ll.length,0);
        
        //insert
        ll.append("1st".to_string());
        ll.append("2nd".to_string());
        ll.append("3rd".to_string());
        println!("{:?}",ll);
        assert_eq!(ll.length,3);
        let third=ll.pop();
        let second=ll.pop();
        let first=ll.pop();
        println!("{}, {}, {}",first.unwrap(), second.unwrap(), third.unwrap());
        assert_eq!(ll.length,0);
    }

    #[test]
    fn linked_list_box() {
        //create
        let mut ll=linked_list_box::LinkedList::new();
        
        ll.push_front(3);
        ll.push_back(8);
        ll.push_front(1);
        println!("{:?}",ll);
        assert!(true);
    }
}