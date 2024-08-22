// String is growable and mutable while str is not

fn print_str(string_ : &str){
    println!("{}", string_);
}

fn print_string(string_ : String){
    println!("{}", string_);
}


fn main(){

    let str_ = "Hello there!";
    print_str(str_);

    let new_string_ = String::from("I am Sid.");
    print_string(new_string_);

}

