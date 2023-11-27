mod linked_list;

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
        println!("{:#?}",ll);
        assert_eq!(ll.length,3);
    }
}