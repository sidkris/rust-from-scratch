
fn main() {

    // sequence (like a python list)
    let sample_sequence = vec![1, 2, 3, 4, 5];

    for num in &sample_sequence {
        println!("{}", num)
    }


    // map / hashmap (like a python dictionary)
    let mut sample_map = std::collections::HashMap::new();
    sample_map.insert("France", "Paris");

    println!("{:?}", sample_map);


}