use std::fmt;

#[derive(Clone)]
pub struct Person {
    id: i32,
    pub name: String,
    pub age: i32,
}

impl Person {
    pub fn new(id: i32, name: String, age: i32) -> Person {
        Person { id, name, age }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person {{ id: {} , name: {} , age: {} }}", self.id, self.name, self.age)
    }
}
