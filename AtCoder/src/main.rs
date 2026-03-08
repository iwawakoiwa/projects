use proconio::input;

fn main(){
    input!{
        a:i16,
        b:i16,
    }
    if a*b%2 == 0{
        println!("Even");
    }else{
        println!("Odd");
    }
}
