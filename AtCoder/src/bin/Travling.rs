use proconio::input;

fn main() {
    input! {
        n:i32
    }
    let mut ex_xy:[i32;2];
    for _ in 0..n {
        input! {
            t:i32,
            mut xy_vec:[i32;2]
        }
        println!("{},{:?}",t,xy_vec);
        let mut say: bool = false;
        ex_xy = [xy_vec[0], xy_vec[1]];
        'other :for x in 0..=t{
            
            if ex_xy[0] > xy_vec[0] {
                xy_vec[0] += 1;
            }
            else if ex_xy[0] < xy_vec[0] 
            {
                xy_vec[0] -= 1;
            }
            else if ex_xy[0] == xy_vec[0] 
            {
                for _ in 0..=(t-x){
                if ex_xy[1] > xy_vec[1] {
                    xy_vec[1] += 1;
                }
                else if ex_xy[1] < xy_vec[1] 
                {
                    xy_vec[1] -= 1;
                }
                else if ex_xy[1] == xy_vec[1] 
                {
                    say = true;
                    break 'other;
                }
                } 
            }
        }
        if say == true {
            println!("yes")
        }
        else {
            println!("No")
        }
    }
}