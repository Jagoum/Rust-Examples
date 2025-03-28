use crate::reverse_array::SIZE;

pub fn remove_index( index: i32, mut arr: [i32; SIZE], num_of_elem: i32) -> ([i32;SIZE], i32){
    let mut remove = false;
    print!("\nResultant Array: [ ");
    for i in 0..(num_of_elem-1) as usize {
        if i == index as usize{
            arr[i] = arr[i+1];
            remove = true;
        }
        if remove{
            arr[i] = arr[i+1];
        }
        print!("{}, ",arr[i]);
    }
    print!("]");
    (arr, num_of_elem - 1)
}