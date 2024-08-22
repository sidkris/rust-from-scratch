#[derive(Debug)]

struct Person{
    name : String,
    age : u8,
}

impl Person{
    fn new(name : String, age : u8) -> Person {
    Self{
            name,
            age,
        }
    }

    fn change_person_name(&mut self, new_name : &str){
        self.name = new_name.to_string();
    }
}


fn main() {

    let sid = Person{
        name : "Sid".to_string(),
        age : 30
    };

    println!("{} is {} years old.", sid.name, sid.age);

    let mut new_person = Person::new(
        "Sid Krishnan".to_string(),
        34
    );

    println!("{} is {} years old.", new_person.name, new_person.age);
    
    new_person.change_person_name("S Krishnan");
    println!("New Name : {}", new_person.name);

}