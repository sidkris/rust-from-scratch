use std::io;

fn main(){

    print_name();
    height_check(186);
    age_check(30);
    while_loop();
    for_loop();
    for_loop_2();
    match_greetings("Hello");
    simple_unit_function(&[1, 2, 3, 4, 5]);
    
    let chunks = split_strings("Hello, World".to_string(), ',', 0);
    println!("Split String : {}", chunks);

}

// ==========================================================================

fn print_name() {

    println!("Please enter your name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();

    println!("Hello there, {}!", name);

}

// ==========================================================================

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

// ==========================================================================

fn age_check(age : i32){

    println!("{}", if age <= 18 {"you are a teenager!"} else {"you are an adult!"});

}


fn while_loop(){

    let mut i : i32 = 0;

    while i <= 10 {
        println!("i = {}", i);
        i += 1;
    }

}

// ==========================================================================

fn for_loop(){

    for i in 11..21 {
        println!("i = {}", i);
    }

}

// ==========================================================================

fn for_loop_2(){

    let numbers = vec![101, 102, 103, 104, 105];

    for number in numbers{
        println!("{}", number);
    }

}

// ==========================================================================

fn match_greetings(greeting : &str){

    match greeting{

        "Hello" => println!("Hello there!"),
        "Bye" => println!("Talk to you later!"),
        _ => println!("Sorry, could you please say that again?"),

    }

}

// ==========================================================================

// Unit Functions do not return any value(s)
fn simple_unit_function(numbers : &[i32]){

    let mut sum = 0;

    for number in numbers{
        sum += number;
    }

    println!("The sum of the numbers is {}", sum);

}

// ==========================================================================

fn split_strings(input_string : String, delimiter : char, chunk_index : usize) -> String {

    let parts : Vec<&str> =  input_string.split(delimiter).collect();
    let result : Option<&&str> = parts.get(chunk_index);

    result.expect("Oops, that didn't work..!").to_string()
    
}

// ==========================================================================

