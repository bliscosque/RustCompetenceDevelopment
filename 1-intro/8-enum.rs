#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}


#[derive(Debug)]
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write (String),
    ChangeColor (i32,i32,i32),
}

fn main() {
    let home=IPAddr::V4(String::from("127.0.0.1"));
    let loopback=IPAddr::V6(String::from("::1"));
    println!("{:?}",home);
    println!("{:?}",loopback);

    let msg1:Message=Message::Move{x:1,y:2};
    println!("{:?}",msg1);

    //option enum
    let five=Some(5);
    let six=plus_one(five);
    let none=plus_one(None);

    println!("six: {:?}",six);
    println!("none: {:?}",none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
