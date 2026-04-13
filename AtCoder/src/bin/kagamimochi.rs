use proconio::input;

fn main() {
    input!{
        n: usize,
        mut d: [i32; n],
    }
    d.sort();
    d.reverse();
    let mut newd = d[0];
    let mut count = 1;
    for i in 0..n {
        if newd > d[i] {
            newd = d[i];
            count += 1;
        }
    }
    println!("{count}");
}