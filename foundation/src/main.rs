use std::io;

fn main(){

    print_name();

}


fn print_name() {

    println!("Please enter your name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    // Print a greeting message
    println!("Hello there, {}!", name);


}

