pub const SIZE: usize = 50;
pub fn reverse_array(num_of_elem: i32, arr: [i32; SIZE]) -> [i32;SIZE]{
    let mut j = (num_of_elem - 1) as usize;
    let mut i = 0 as usize;
    let mut arr = arr;
    print!("\nRevesed Array: [ ");
    while j > i {
        arr[i] = arr[i]+ arr[j];
        arr[j] = arr[i] - arr[j];
        arr[i] = arr[i] - arr[j];
        i += 1;
        j -= 1;
    }
    for i in 0..num_of_elem as usize {
        print!("{}, ", arr[i]);
    }
    print!("]");
    arr
}




