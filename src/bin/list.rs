use std::rc::Rc;

use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[test]
fn list_test() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    assert_eq!(1, Rc::strong_count(&a));

    let b = Cons(12, Rc::clone(&a));
    assert_eq!(2, Rc::strong_count(&a));

    {
        let c = Cons(13, Rc::clone(&a));
        assert_eq!(3, Rc::strong_count(&a));
    }

    assert_eq!(2, Rc::strong_count(&a));
}