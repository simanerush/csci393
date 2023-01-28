use std::env;
use os_elements::spin_s;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let msg = &args[1];
        loop {
            println!("{msg}");
            spin_s(1);
        }
    } else {
        println!("usage: cpu <msg>");
    }
}


