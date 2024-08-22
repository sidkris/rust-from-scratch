#[derive(Debug)]

struct Person{
    name : String,
    age : u8,
}

fn main() {

    let sid = Person{
        name : "Sid".to_string(),
        age : 30
    };

    println!("{} is {} years old.", sid.name, sid.age);

}