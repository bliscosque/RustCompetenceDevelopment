pub struct LinkedList<T> ( // using () means that the fiels are pub by default, instead of {}
    Option<(T,Box<LinkedList<T>>)>
);

//alternative code for LL struct:
// pub struct LinkedList<T> {
//     pub node: Option<(T, Box<LinkedList<T>>)>
// }
