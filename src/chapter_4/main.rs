fn main(){
    let mut nums = vec![1,2,3,4,5,6,7,8,10];
    double_values(&mut nums);
    println!("{:?}",nums);
    let name = String::from("Richard");
    let size = string_length(&name);
    println!("size:{}",size);
}

fn double_values(nums: &mut Vec<i32>) {
    for element in nums.iter_mut(){
        *element *= 2
    }

}

fn string_length(s: &String) -> usize { 
    s.len()
 }



