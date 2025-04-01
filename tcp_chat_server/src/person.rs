use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Person {
    id: u32,
    name: String,
    age: u32,
}

pub struct PersonManager {
    next_id: u32,
    id_to_person: HashMap<u32, Person>,
    name_to_person: HashMap<String, Person>,
}

impl PersonManager {
    pub fn new() -> Self {
        PersonManager {
            next_id: 1,
            id_to_person: HashMap::new(),
            name_to_person: HashMap::new(),
        }
    }

    pub fn add_person(&mut self, name: String, age: u32) {
        let id = self.next_id;
        let person = Person { id, name: name.clone(), age };

        // Insert into both HashMaps
        self.id_to_person.insert(id, person.clone());
        self.name_to_person.insert(name, person);

        // Increment the next_id
        self.next_id += 1;
    }

    pub fn get_by_id(&self, id: u32) -> Option<&Person> {
        self.id_to_person.get(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Person> {
        self.name_to_person.get(name)
    }
}

fn main() {
    let mut manager = PersonManager::new();

    manager.add_person("Alice".to_string(), 25);
    manager.add_person("Bob".to_string(), 30);

    // Access by ID
    if let Some(person) = manager.get_by_id(1) {
        println!("Person with ID 1: {:?}", person);
    }

    // Access by Name
    if let Some(person) = manager.get_by_name("Alice") {
        println!("Person with name Alice: {:?}", person);
    }
}