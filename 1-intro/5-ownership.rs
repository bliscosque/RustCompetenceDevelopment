fn main() {
    let s=String::from("hello");
    takes_ownership(s);
    //println!("{s}"); won't work because ownership of s went to the function and was dropped.
    
    let i=10;
    make_copy(i);
    println!("{i}"); //int values are copied to functions, so no issues...
    
    let x: Box<i32> = Box::new(5); // go to heap
    let mut y=Box::new(1);
    *y=4; //change value
    println!("{y}"); //print macro does not require *

    // partial copy sample
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person=Person {
        name: String::from("I'm a person"),
        age: Box::new(20),
    };

    println!("{:?}",person);

    let Person {name,ref age} = person; //destructuring person to name and age
    // after this point, can access name, age, person.age (it was passed a reference), but not
    // person.name
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn make_copy(i: i32) {
    println!("{i}");
}
