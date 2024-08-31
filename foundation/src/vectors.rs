fn main() {

    let nums = vec![1, 2, 3, 4, 5];

    for num in nums.iter(){
        println!("{}", num);
    }

    let num_slice = &nums[0..2];
    println!("{:?}", num_slice);

}