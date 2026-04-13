use proconio::input;
fn main() {
    input! {
        n:i32,
    }
    let mut input = 0;
    for i in 0..=n {
        if i % 2 == 1 {
            input += i;
        }
    }
    println!("{input}")
}