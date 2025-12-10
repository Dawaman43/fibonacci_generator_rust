use std::io;
use num_bigint::BigUint;
use num_traits::{One, Zero};


fn main() {
   println!("Fibonacci series");
   println!("Enter a number toget fibonacci series:");

    let mut fibonacci = String::new();

    io::stdin()
        .read_line(&mut fibonacci)
        .expect("Please input read line");

    let n: u32= match fibonacci.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number");
            return;
        }
    };

   let mut a: BigUint = Zero::zero();
   let mut b: BigUint = One::one();

   if n == 0 {
    println!("Fibonacci = 0");
   } else if n == 1 {
       println!("Fibonacci series =  1");
   }

   for _ in 2..=n {
    let next = &a + &b;
    a = b;
    b = next;
   }

   println!("Fibonacci ({}) = {}",n, b);
}

