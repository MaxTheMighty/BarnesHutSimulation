use crate::List::{Cons,Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    let _list = Cons(1, Box::from(Cons(2, Box::from(Cons(3, Box::from(Nil))))));
    let b = Box::new(5);
    println!("b = {}",b);
    let b_ptr = Box::into_raw(b);
    println!("b_raw = {:?}",b_ptr);
    let _b = unsafe { Box::from_raw(b_ptr) };
}