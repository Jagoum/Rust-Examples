mod get_values;
mod insert_into_array;
mod remove_from_array;
mod reverse_array;

use get_values::{get_array, get_nums};
use insert_into_array::add_element;
use remove_from_array::remove_index;
use reverse_array::reverse_array;

fn main() {
    println!("\nPlease enter the number element of the array [1 <= number of elements <= 50]");
    let mut num_of_elem = get_nums();
    let mut arr = get_array(num_of_elem);
    if num_of_elem < 1 {
        return println!("\nNumber of element cannot be less than 1");
    }
    loop {
        println!("\nEnter\n1=>To reverse the array\n2=>To remove an element from the array\n3=>To add an element into the array\n4=>To Exit");

        let opt = get_nums();

        match opt {
            1 => {
                arr = reverse_array(num_of_elem, arr);
            }
            2 => {
                println!("\nEnter the position  to remove  element from array [ 0 < index < number of elements ]");
                let index = get_nums();
                (arr, num_of_elem) = remove_index(index - 1, arr, num_of_elem);
            }
            3 => {
                println!("\nEnter the position to insert new element [ 0 < index < number of elements ]");
                let index = get_nums();
                println!("\nEnter the element you would like to insert");
                let element = get_nums();
                (arr, num_of_elem) = add_element(index - 1, element, num_of_elem, arr);
            }
            4 => {
                break;
            }
            _ => println!("\nSorry, enter one of the above options to proceed!"),
        }
        println!();
    }
}
