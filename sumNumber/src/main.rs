fn main() {
    let  array:[u32;8] = [1,3,42,5,6,7,68,19];   
    let sum = sum_compute(&array);
    match sum{
        None=>println!("this is a error."),
        Some(value)=>println!("The summer value of this array is {}", value)
    }
}


fn sum_compute(p_array:&[u32])->Option<u32>{
    let mut sum: u32 = 0;
    for i in p_array.iter(){
        sum += i
    }
    Some(sum)
}
