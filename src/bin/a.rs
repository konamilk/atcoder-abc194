use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
    a:i32,
    b:i32
    }

    if a + b >= 15 && b>= 8 {
        println!("1");
        return
    }
    else if a + b >= 10 && b >= 3 {
        println!("2");
        return
    }
    else if a + b >= 3 {
        println!("3");
        return
    }
    else {
        println!("4");
    }
}
