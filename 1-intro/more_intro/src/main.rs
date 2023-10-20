use rand::Rng;
fn main() {
    //random
    
    let random_num=rand::thread_rng().gen_range(1..101);
    println!("random from 1..101: {}",random_num);
}
