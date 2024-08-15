use std::io;

fn main(){

    print_name();
    height_check(186);

}


fn print_name() {

    println!("Please enter your name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    println!("Hello there, {}!", name);

}


fn height_check(height : i32){

    if height > 180 {
        println!("you are tall!");
    }

    else if 170 < height && height <= 180 {
        println!("you are pretty tall!");
    }

    else{
        println!("hey, a normal human being! ;)")
    }


}
