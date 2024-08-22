#[derive(Debug)]

struct Person{
    name : String,
    age : u8,
}

impl Person{
    fn new(name : String, age : u8) -> Person {
    Person{
            name,
            age,
        }
    }
}


fn main() {

    let sid = Person{
        name : "Sid".to_string(),
        age : 30
    };

    println!("{} is {} years old.", sid.name, sid.age);

    let new_person = Person::new(
        "Sid Krishnan".to_string(),
        34
    );

    println!("{} is {} years old.", new_person.name, new_person.age);


}