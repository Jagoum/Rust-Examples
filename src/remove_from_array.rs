use crate::reverse_array::SIZE;

pub fn remove_index( index: i32, mut arr: [i32; SIZE], num_of_elem: i32) {
    
    print!("\nResultant Array: ");
    for i in 0..num_of_elem as usize {
        if i == index as usize{
            arr[i] = arr[i+1];
        }
        print!("{} ", arr[i]);
        if i == num_of_elem as usize{
            break;
        }
    }
}