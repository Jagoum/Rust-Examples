use crate::reverse_array::SIZE;

pub fn add_element(index: i32, element: i32, num_of_elem: i32, arr: [i32; SIZE]) /*[i32; SIZE]*/
{
    let mut arr = arr; //  let arr = [ 1, 2, 4, 5, 7, 9 ]
    let mut ri = num_of_elem as usize; // the expected value of ri should be num_of_element which is 6
    let index = index as usize; // let say the user want to remove an element at index 3, there4 index = 3
    let mut n = ri - 1; // n = 6 - 1 = 5
    let mut w = 0; // w is a varaible we will be using to swap, let say its some sort of container

    for _ in 0..num_of_elem {
        if ri != index {
            // here we check if the index is not eqaul to ri which is the right index
            w = arr[ri]; // we assign the element at that index of ri to container w
            arr[ri + 1] = w; // we then put what was in the container w into the empty index of the array which is at position index 6; or position 7
            arr[ri] = arr[n]; // we then assign the value at the index lower than ri which is n to the index ri
        } else {
            arr[index] = element; // when we finally arrive at ri = index the ri will be empty so we now put the element we had at that index
            break; //here i break since i have already had what i am looking for
        }
        n -= 1; //here we decrement both ri and n so that the both will shift to the next indices
        ri -= 1; // at this point ri occupies the empty index n which just move it value to the prevous ri
                 //ni now move to to another index that hold a value and now ri is the one that is empty and we check our if condition agian and the proceedure continues
    }
    println!("\nResultant Array after inserting and element: ");
    for i in 0..=num_of_elem as usize {
        print!("{} ", arr[i]);
    }
}
