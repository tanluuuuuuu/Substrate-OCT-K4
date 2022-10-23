// ====================================
// BÀI 3:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================


fn main(){
    iter_num(100);
}

pub fn iter_num(num: i32) -> bool {

    let num_str = num.to_string();
    let chars: Vec<char> = num_str.chars().collect(); // <-- move occurs because `chars` has type `Chars<'_>`, which does not implement the `Copy` trait
    let len = chars.len();     // <-- `chars` moved due to this method call

    println!("Len = {:?}", len);

    for c in chars {             // <-- ❌ "value used here after move": chars
        println!(">>> {:?}", c);
    }
    return true;
}