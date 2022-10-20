// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1,2,3,5,6,8,10,11];
//         let sub_arr = [6,8,10];
#![allow(unused)]

use core::num;

fn main() {
    let org_arr = vec![1,6,3,5,6,8,10,11];
    let sub_arr = vec![6,8,10];

    // Brute-force
    // Check if every number in sub array appear in original array
    // for num_in_sub_arr in sub_arr {
    //     let mut flag = false;
    //     for num_in_org_arr in org_arr {
    //         if (num_in_sub_arr == num_in_org_arr)
    //         {
    //             flag = true;
    //             break;
    //         }
    //     }
    //     if (!flag)
    //     {
    //         println!("Not subarray");
    //         break;
    //     }
    // }
    // println!("Subarray");

    let mut it = sub_arr.iter();
    let mut number = it.next();
    for (idx, num_in_org_arr) in org_arr.iter().enumerate() {
        match number {
            Some(x) => {
                if (x == num_in_org_arr)
                {
                    number = it.next();
                }
            },
            None => {
                println!("Subarray");
                return;
            }
        }
    }
    println!("Not subarray");
}
