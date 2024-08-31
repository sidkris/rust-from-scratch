// ENUM: 
// An enum, short for enumerator, is a data type in Rust that allows you to define a set of
// possible values. It's used to create custom types that represent distinct variants or cases.
// Enums are powerful because they enable you to encapsulate related values and provide exhaustive
// matching capabilities through the match keyword. (Source : Coursera)

// VARIANT:
// A variant is a specific case within an enum. Each enum can have multiple variants, which may 
// optionally contain associated data. For example, in the text provided, there are enums like Shape,
// with two variants: Circle and Square. The Circle variant has an associated radius value, while
// the Square variant does not carry any additional data. (Source : Coursera)


fn main(){

    let file_size = FileSize::Bytes(1024);
    let formatted_size = file_size.format_size();
    println!("{}", formatted_size);

    let square = Shape::Square(10.0);
    let square_area = square.calculate_area();
    println!("Area of the square is {}", square_area);

    let city = MegaCities::Mumbai;
    megacity_check(city);

}


enum Shape {
 
    Circle(f64), // Radius of a circle
    Square(f64), // Length of one side for a square

}

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            Self::Bytes(bytes) => format!("{} bytes", bytes),
            Self::Kilobytes(kilobytes) => format!("{} KB", kilobytes),
            Self::Megabytes(megabytes) => format!("{} MB", megabytes),
            Self::Gigabytes(gigabytes) => format!("{} GB", gigabytes),
        }
    }
}

impl Shape {
    fn calculate_area(&self) -> f64 {
        match self{
            Self::Circle(radius) => 3.14 * (radius * radius),
            Self::Square(side) => side * side,
        }
    }
}

// using enums as a datatype
#[derive(Debug)]
enum MegaCities {
    Mumbai,
    NewYorkCity,
    NewDelhi,
    Tokyo,
    Jakarta,
    Shanghai,
    Beijing,
}

struct Cities{
    country : String,
    city : MegaCities
}


fn megacity_check(city: MegaCities) {
    match city {
        MegaCities::Mumbai 
        | MegaCities::NewYorkCity 
        | MegaCities::NewDelhi 
        | MegaCities::Tokyo 
        | MegaCities::Jakarta 
        | MegaCities::Shanghai 
        | MegaCities::Beijing => {
            println!("{:?} is a megacity!", city);
        },
    }
}
