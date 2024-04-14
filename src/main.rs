use std::env;

fn main() {
    
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let args = args;
    
    for arg in args {
        println!("{}", arg);
    }

    // tbc

}
