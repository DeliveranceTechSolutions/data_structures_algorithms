pub mod stack;
pub mod queue;
use crate::stack::StackActions;
use crate::queue::QueueActions;
use crate::queue::QueueReader;
use std::fmt;
use std::process::{id, Command};


fn main() {
    let mut q = queue::new::<u32>();
    let mut thread = Command::new("sh");
    
    q.push(0);
    q.push(1);
    q.push(2);
    q.push(3);
    // println!("{:?}", q.first());
    // println!("{:?}", q.last());
    q.insert(100, 3);
    q.view();
    // let mut res = thread.arg("cat").arg(format!("/proc/{:?}/status", id())).output();
    // match res {
    //     Ok(T) => println!("{:?}", res),
    //     Err(T) => println!("Error") 
    // }

}
