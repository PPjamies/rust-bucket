use std::cell::RefCell;
use std::rc::Rc;

use crate::ChangeableList::{Cons, Nil};

#[derive(Debug)]
enum ChangeableList {
    Cons(Rc<RefCell<i32>>, Rc<ChangeableList>),
    Nil,
}

#[test]
fn changeable_list_test() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}