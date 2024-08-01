use std::fmt;
use std::fmt::Formatter;

pub struct Person {
    pub fname: String,
    pub lname: String,
    pub age: i32,
    pub sex: String,
}

impl Person {
    pub fn new(p_fname: &str, p_lname: &str, p_age: &i32, p_sex: &str) -> Person {
        Person {
            fname: p_fname.to_string(),
            lname: p_lname.to_string(),
            age: *p_age,
            sex: p_sex.to_string(),
        }
    }

    pub fn copy(&self) -> Self {
        Self::new(&self.fname, &self.lname, &self.age, &self.sex)
    }

    pub fn set_age(&mut self, p_age: &i32) {
        self.age = *p_age
    }

    fn get_age(&self) -> &i32 {
        &self.age
    }

    pub fn print(&self) {
        println!("{:?} {:?} {:?} {:?}", self.fname, self.lname, self.age, self.sex)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_age())
    }
}