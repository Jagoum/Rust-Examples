use crate::reverse_array::SIZE;

pub fn remove_index( index: i32, arr: [i32; SIZE], num_of_elem: i32) {
    
    print!("\nResultant Array: ");
    for i in 0..num_of_elem as usize {
        if i == index as usize{
            continue;
        }
        print!("{} ", arr[i]);
        if i == num_of_elem as usize{
            break;
        }
    }
}