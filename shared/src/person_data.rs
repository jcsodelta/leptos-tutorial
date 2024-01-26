use crate::models::person::Person;

pub struct PersonData {
    next_person_id: u32,
    persons: Vec<Person>,
}

impl PersonData {
    pub fn new() -> PersonData {
        let mut person_data = PersonData {
            next_person_id: 1,
            persons: vec![],
        };

        for person_name in PERSON_NAMES {
            person_data.create_person(person_name);
        }

        person_data
    }

    pub fn create_person(&mut self, person_name: &'static str) -> &Person {
        let person = Person::new(self.next_person_id, person_name, 0);
        self.persons.push(person);
        self.next_person_id += 1;

        self.persons
            .last()
            .expect("self.persons should not be empty")
    }

    pub fn get_persons(&self) -> &Vec<Person> {
        &self.persons
    }

    pub fn increase_age(&mut self, id: u32) {
        //self.persons.iter().map(|person| person.get_id = id)
    }
}

const PERSON_NAMES: [&str; 26] = [
    "Alberto",
    "Bento",
    "Carlos",
    "Diego",
    "Evandro",
    "Fábio",
    "Gustavo",
    "Humberto",
    "Iago",
    "João",
    "Kleber",
    "Luiz",
    "Marcelo",
    "Neto",
    "Osvaldo",
    "Pedro",
    "Quitério",
    "Rogério",
    "Simão",
    "Tiago",
    "Umberto",
    "Vagner",
    "Wagner",
    "Xerxes",
    "Yago",
    "Zeca",
];
