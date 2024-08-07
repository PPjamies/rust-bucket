use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn coercion(name: &str) -> String {
    format!("Hello, {}", name)
}

#[test]
fn mybox_test() {
    let x = 5;
    let my_box = MyBox::new(x);

    assert_eq!(x, *my_box)
}

#[test]
fn coercion_test() {
    let expected = "Hello, Paz".to_string();

    let name = MyBox::new(String::from("Paz"));
    let actual = coercion(&name);

    assert_eq!(expected, actual)
}