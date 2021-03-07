use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
        n: usize,
        a: [i64; n]
    }


    let mut sum = 0;
    let mut sum2 = 0;
    let mut squared_sum = 0;

    for i in 0..n {
        sum += a[i];
    }

    squared_sum = sum * sum;

    for i in 0..n{
        sum2 += a[i] * a[i];
    }

    let ans = (n as i64) * sum2 - squared_sum;

    println!("{}", ans)
}
