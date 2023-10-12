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
}
