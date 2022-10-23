// ====================================
// BÀI 4:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================

fn main() {
    let mut v = vec![1, 2, 3];

    v = go(v);

    // still need v here, so I can't pass ownership to the "go' method above
    println!("{}", v.len())
}

fn go(mut v: Vec<i32>) -> Vec<i32> {
    for i in &v {
        println!("{}", i);
    }
    v.push(4);
    return v;
}