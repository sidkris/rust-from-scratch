#[derive(Debug)]

struct Person{
    name : String,
    age : u8,
}

fn main(){

    println!("{:?}", Person{name : "Sid".to_string(),
                            age : 30});

}