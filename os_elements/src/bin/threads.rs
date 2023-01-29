use std::env;
use std::thread;
use std::sync::Mutex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = Mutex::new(0);
    if args.len() == 3 {
        let init_val = *n.lock().unwrap();
        let loop_count: usize = args[1].parse().expect("number of loops should be an int");
        let thread_count: usize = args[2].parse().expect("number of threads should be an int");
        println!("initial value {init_val}, each of {thread_count} thread's will loop {loop_count} times");

        thread::scope(|s| {
            for _ in 0..thread_count {
                s.spawn(|| {
                    for _ in 0..loop_count {
                        let mut guard = n.lock().unwrap();
                        *guard += 1;
                    }
                });
            }
        });
        let final_value = *n.lock().unwrap();
        println!("final counter value {final_value}");
    } else {
        println!("usage: threads <num_loops> <num_threads>");
    }
}