use std::io;

use crate::reverse_array::SIZE;

pub fn get_nums() -> i32 {

let mut  num = String::new();
loop{
io::stdin().read_line(&mut num).expect("Failed to read line");

let num: i32 = match num.trim().parse() {
    Ok(val) => {val},
    Err(_) => {println!("Please enter an interger number");
continue ;},
};
let ok = num;
if ok == num{
    break num;
}
}   
}
pub fn get_array(num_of_elem: i32) -> [i32; SIZE]{
    let mut arr: [i32; SIZE] = [0;SIZE];
    println!("\nPlease enter the elements of the array");
    for i in 0..num_of_elem as usize {
        arr[i] = get_nums();
    }
    print!("\nInitail Array: ");
    for i in 0..num_of_elem as usize{
        print!("{} ",arr[i]);
    }    
arr
}