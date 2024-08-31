fn main(){

    let sentence = "Hi, my name is Sid!".to_string();

    println!("{}", &sentence[0..5]);

// match characters

for c in sentence.chars() {

    match c {

        'a' | 'e' | 'i' | 'o' | 'u' => println!("hey, I found the vowel {}!", c),

        _ => continue,

    }


}

}