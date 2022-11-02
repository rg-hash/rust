mod utils;
mod utils::*;
fn main(){
let mut a=20;
a=12;
  println!("hello world");
  println!("weight of {} is {}",2,calculate_weight(5.0));
 
 let inp=Input{
   a:20,
   b:30
 }
 let pq=utils::sum(inp)
 println!("{}",pq);
  }
  fn calculate_weight(m: f32)->32{
    m*9.81
  }
