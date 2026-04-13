use proconio::input;

fn main() {
    input! {
        n:usize,
        plan_input:[(i32,i32,i32);n],
    }
    let mut plan:Vec<(i32,i32,i32)> =vec!((0,0,0));
    plan.extend(plan_input.clone());
    let mut say = true;
    for i in 0..n 
    {
        let j = i + 1;
        let dt = plan[j].0 - plan[i].0;
        let tim = (plan[i].1 - plan[j].1).abs() + (plan[i].2 - plan[j].2).abs();

        if tim > dt || (dt - tim) %2 != 0
        {
            say = false;
            break;
        }
    }
    if say == true 
    {
        println!("Yes")
    }
    else 
    {
        println!("No")
    }
}