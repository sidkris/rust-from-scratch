fn main() {

    let nums = vec![1, 2, 3, 4, 5];

    for num in nums.iter(){
        println!("{}", num);
    }

    let num_slice = &nums[0..2];
    println!("{:?}", num_slice);

    mutable_vec();

}


fn mutable_vec(){

    let mut num_vec = vec![1, 2, 3];

    num_vec.extend([4, 5, 6, 7, 8, 9, 10]); // 'push' for adding a single element to the end of the vec

    println!("New Vec : {:?}", num_vec);
    println!("\nThe first element of the vector is {:?}", num_vec.first().unwrap());
    println!("\nThe last element of the vector is {:?}", num_vec.last().unwrap());

}