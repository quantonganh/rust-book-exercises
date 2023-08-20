use std::ptr::addr_of;

fn main() {
    let s1 = "hello";
    let addr_s1 = addr_of!(s1);
    println!("Address of s1: {:p}", addr_s1);

    let s2 = "world";
    let addr_s2 = addr_of!(s2);
    println!("Address of s2: {:p}", addr_s2);

    let h = String::from("Heap-allocated string");
    let addr_h = addr_of!(h);
    println!("Address of h: {:p}", addr_h)
}
