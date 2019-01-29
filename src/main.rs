#![feature(test)]

extern crate test;
extern crate num;

use std::io;
use std::process;
use num::{BigUint, FromPrimitive, Zero, One};

fn main() {
    let x = read_start_number();
    do_collatz(x);
}

fn read_start_number() -> u64 {
    let start_number = loop {
        let mut input = String::new();
        println!("Enter the number to start with:");
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        match input.trim().parse::<u64>() {
            Ok(num) => {
                if num > 1 {
                    break num
                } else {
                    println!("Only integer values larger than 1 allowed!");
                }
            },
            Err(_) => println!("This is not a 64-bit integer!"),
        };
    };
    start_number
}

fn do_collatz(mut x: u64) {
    let mut i = 0;
    loop {
        if x == 1 {
            break
        }
        else if x % 2 == 0 {
            x = x / 2;
        } else {
            x = match x.checked_mul(3) {
                Some(x) => x,
                None => {
                    println!("Integer Overflow");
                    process::exit(0);
                },
            };
            x=x+1;
        }
        //println!("{}", x);
        i+=1;
    }
    println!("It took {} iterations to reach one.", i);
}

fn do_collatz_without_overflow_check(mut x: u64) {
    let mut i = 0;
    loop {
        if x == 1 {
            break
        }
        else if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3*x+1;
        }
        //println!("{}", x);
        i+=1;
    }
    println!("It took {} iterations to reach one.", i);
}

fn do_collatz_with_bigints(mut x: BigUint) {
    let mut i = 0;
    loop {
        if x == BigUint::one() {
            break
        }
        else if x.modpow(&One::one(), &BigUint::from_u32(2).unwrap()) == BigUint::zero() {
            x = x / BigUint::from_u32(2).unwrap();
        } else {
            x = BigUint::from_u32(3).unwrap()*x+BigUint::one();
        }
        //println!("{}", x);
        i+=1;
    }
    println!("It took {} iterations to reach one.", i);
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;

    #[bench]
    fn bench_collatz(b: &mut Bencher) {
        b.iter(|| do_collatz(12341451251512515));
    }

    #[bench]
    fn bench_collatz_without_overflow_check(b: &mut Bencher) {
        b.iter(|| do_collatz_without_overflow_check(12341451251512515));
    }

     #[bench]
     fn bench_do_collatz_with_bigints(b: &mut Bencher) {
         b.iter(|| do_collatz_with_bigints(BigUint::from_u64(12341451251512515).unwrap()));
     }
}
