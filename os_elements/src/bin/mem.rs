use std::ptr;
use os_elements::spin_s;

fn main() {
    // allocate an int on the heap
    let mut p: Box<u32> = Box::new(0);

    loop {
        // print p and its address
        println!("({:?}) p: {p}", ptr::addr_of_mut!(p));
        *p += 1;
        spin_s(1);
    }
}