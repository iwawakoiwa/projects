use proconio::input;

fn main() {
    input! {
        n:i32
    }
    let mut ex_xy:[i32;2] = [0,0];
    for i in 0..n {
        input! {
            t:i32,
            xy_vec:[i32;2]
        }
        println!("{},{:?}",t,xy_vec);
        ex_xy = [xy_vec[0], xy_vec[1]];
    }

}