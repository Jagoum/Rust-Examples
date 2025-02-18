mod reverse_array;
mod get_values;
use get_values::{get_array, get_nums};
use reverse_array::{add_element, remove_index, reverse_array, SIZE};

fn main() {
    println!("\nPlease enter the number element of the array [1 <= number of elements <= 50]");
    let num_of_elem = get_nums();
    let arr= get_array(num_of_elem);

    println!("Please Enter\n1=>To reverse the array\n2=>To remove an element from the array\n3To add an element into the array");
    
    let opt = get_nums();

    match opt {
        1 =>{reverse_array(num_of_elem, arr);},
        2 => {    println!("\nPlease enter the index at which you would like to remove an element from the initail array");
        let index =get_nums();
        remove_index(index, arr, num_of_elem);},
        3 => {     println!("\nPlease enter the index at which you would like to insert an element");
        let index = get_nums();
        println!("\nEnter the element you would like to insert");
        let element = get_nums();
        add_element(index, element, num_of_elem, arr);},
        _ => println!("Please enter one of the above options"),
        
    }








    
}

