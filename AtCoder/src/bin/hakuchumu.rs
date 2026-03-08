use proconio::input;
fn main() {
    input! {
        mut s:String,
    }
    let ward = ["dream","dreamer","erase","eraser"];
    loop {
        let mut matched = false;
        for i in 0..4 {
            if s.ends_with(ward[i]) {
                let new_s = s.len()-ward[i].len();
                s.truncate(new_s);
                matched = true;
                break;
            }
        }
        if s.is_empty(){
            println!("YES");
            return;
        }
        if !matched {
            println!("NO");
            return;
        }
    }
}