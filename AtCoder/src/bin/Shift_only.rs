use proconio::input;

fn main() {
    input! {
    n: usize,
    mut a : [i32; n],
    }
    let mut t = 0;
    loop {
         if a.iter().all(|x| x % 2 == 0){
                a.iter_mut().for_each(|x| *x /= 2);
                t += 1;
            }else {
                println!("{t}");
                return;
            }
    }
}