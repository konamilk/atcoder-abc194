use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
        n: usize,
        ab: [(i32,i32);n]
    }

    let mut ans = std::i32::MAX;

    for i in 0..n {
        for j in 0..n {
            let  mut time = 0;
            if i == j{
                time = ab[i].0 + ab[j].1;
            }
            else {
                time = std::cmp::max(ab[i].0, ab[j].1);
            }

            ans = std::cmp::min(ans, time)
        }
    }

    println!("{}", ans)
}
