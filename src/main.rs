use std::env;

fn main() {
    let count = env::args().count();
    if count == 1 {
        print!("\n")
    } else {
        for arg in env::args().skip(1).take(count-2) {
            print!("{} ", arg);
        }
        println!("{}", env::args().nth(count-1).unwrap());
    }
}
