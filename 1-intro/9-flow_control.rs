enum Coin {
     Penny,
     Nickel,
     Dime,
     Quarter,
}

fn main() {
    let mut counter=0;

    let result = loop {
    	counter+=1;
    	if counter==10 {
        	break counter*2; //result=20
    	}
    };

    println!("result: {result}");

    let c=Coin::Quarter;
    let v=value_in_cents(c);
    println!("{v}");
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

