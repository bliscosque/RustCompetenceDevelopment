fn main() {
    let s=String::from("hello");
    takes_ownership(s);
    //println!("{s}"); won't work because ownership of s went to the function and was dropped.
    
    let i=10;
    make_copy(i);
    println!("{i}"); //int values are copied to functions, so no issues...
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn make_copy(i: i32) {
    println!("{i}");
}
