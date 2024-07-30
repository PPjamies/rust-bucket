use std::fmt;
use std::fmt::Formatter;

// #[derive(Debug)]
struct Person {
    fname: String,
    lname: String,
    age: i32,
    sex: String,
}

impl Person {
    fn new(p_fname: &str, p_lname: &str, p_age: &i32, p_sex: &str) -> Person {
        Person {
            fname: p_fname.to_string(),
            lname: p_lname.to_string(),
            age: *p_age,
            sex: p_sex.to_string(),
        }
    }

    fn copy(&self) -> Self {
        Self::new(&self.fname, &self.lname, &self.age, &self.sex)
    }

    fn set_age(&mut self, p_age: &i32) {
        self.age = *p_age
    }

    fn get_age(&self) -> &i32 {
        &self.age
    }

    fn print(&self) {
        println!("{:?} {:?} {:?} {:?}", self.fname, self.lname, self.age, self.sex)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_age())
    }
}

fn main() {
    println!("\nTesting Structs...");
    let mut person = Person::new(
        "PJ",
        "Jindrich",
        &27,
        "female",
    );
    person.print();

    person.set_age(&28);
    person.print();

    let another_person = person.copy();
    another_person.print();

    println!("Prints only the age: {:?}", another_person);
}