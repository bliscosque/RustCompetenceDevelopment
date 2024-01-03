fn new_stack(max_size: usize) -> Vec<u32> {
    let vec:Vec<u32> = Vec::with_capacity(max_size);
    vec
}

fn push(stack: &mut Vec<u32>, item: u32, max_size: usize) {
    if stack.len()==max_size {
        println!("Stack is full");
    } else {
        stack.push(item);
    }
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped=stack.pop();
    poped
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n=String::new();
    std::io::stdin().read_line(&mut n).expect("failed to read input.");
    let n=n.trim().parse().expect("invalid input");
    n
}

fn main() {
    println!("Enter stack size: ");
    let size_stack=input();
    let mut stack=new_stack(size_stack as usize);

    loop {
        println!("***MENU***");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Display");
        println!("4. Size");
        println!("5. Exit");
        println!("\nEnter your choice");
        let choice=input();
        match choice {
            1 => {
                println!("Enter value to be inserted: ");
                let item=input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => println!("Element poped: {:?}", pop(&mut stack)),
            3 => println!("Stack: {:?}",stack),
            4 => println!("Size: {}",size(&stack)),
            5 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Wrong Option! Try again!"),
        }
    }
}
