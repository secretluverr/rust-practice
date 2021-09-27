mod error;
mod hashmap;

use std::{cell::RefCell, option::Option, rc::Rc};
fn main() {
    let world = Rc::new(RefCell::new("World".to_string()));
    let n = hashmap::empty_node::<String>();
    n.borrow_mut().push(String::from("hello"), world);
    println!("{:#?}", n);
}
