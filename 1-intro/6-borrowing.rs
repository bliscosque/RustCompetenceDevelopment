fn main() {
    let s1=String::from("hello");
    let len=calculate_len(&s1); //borrow by ref 
    println!("Len of {s1}, {len}");

    //mutable
    let mut s2=String::from("hello");
    change(&mut s2); //s2 remains owner
    println!("{s2}");

    let ref_to_nothing=no_dangle();
}

fn calculate_len(s:&String) -> usize { //borrow 
    s.len()
}

fn change(s: &mut String) {
    s.push_str("World")
}

//wont compile...
//fn dangle() -> &String {
//    let s=String::from("hello");
//    &s
//

//this is Ok
fn no_dangle() -> String {
    let s=String::from("Hello");
    s
}
