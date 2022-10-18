// Bài tập 2 : Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện của từ đó trong chuỗi đã cho, 
// và chuỗi không chứa ký tự nhập từ bàn phím. Lưu ý: khong phân biệt viết hoa, viết thường
// Ví dụ: let input = “adbcdaDd”. 
// Nhập s = ‘a’ => in ra kết quả : 2
// Nhập s = ‘d’ => in ra kết quả : 4

#![allow(unused)]

use std::{io::{self, Read}, collections::{HashMap}};

fn main() {
    // Đọc dữ liệu input
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("No input");

    // Xây dựng bảng hash string -> integer
    let mut mapping: HashMap<String, i32> = HashMap::new();
    let input_char_vec: Vec<char> = input.chars().collect();

    for ch in input_char_vec {
        if ch != '\n' && ch != '\r' {
            mapping.entry(ch.to_lowercase().to_string())
                    .and_modify(|num| *num += 1)
                    .or_insert(1);
        }
    }

    loop {
        println!("Enter a character: ");
        let mut char = String::new();
        io::stdin().read_line(&mut char).expect("No input");
        match mapping.get(&char[0..1].to_lowercase().to_string()) {
            Some(number) => println!("{}: {}", &char[0..1], number),
            None => println!("{} not found.", &char[0..1])
        }
    }
}
