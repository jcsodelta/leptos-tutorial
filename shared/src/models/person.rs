use std::fmt;

#[derive(Clone)]
pub struct Person {
    id: u32,
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(id: u32, name: &str, age: u32) -> Person {
        Person { id, name: String::from(name), age }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person {{ id: {} , name: {} , age: {} }}", self.id, self.name, self.age)
    }
}
