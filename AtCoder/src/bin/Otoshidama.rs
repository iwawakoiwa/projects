use::proconio::input;

fn main() {
    input!{
        n:i64,
        y:i64,
    }
    for i in 0..=n {
        for j in 0..=n - i {
            let k = (y - (i*10000 + j*5000))/1000;
                if i + j + k == n{
                    println!("{i} {j} {k}");
                    return;
            }
        }
    }
    println!("-1 -1 -1");    
}