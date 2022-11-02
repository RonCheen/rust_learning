const MAX_POINTS: i32 = 100_000;
use num::complex::Complex;
fn main() {
    println!("Hello, world!");
    let (a, mut b):(bool, bool) = (true, false);
    println!("a={:?}, b={:?}", a,b);
    b = true;
    assert_eq!(a, b);
    let c = a==b;


   let a = (-1.0_f32).sqrt();
   if a.is_nan() {
    println!("未定义数学行为")
   }
   for i in 1..=5 {
    println!("{}", i)
   }
   forever()

}

fn variable(x: i32, y: i32) -> i32 {
    x+y
}

fn dead_end() -> ! {
    panic!("you are going to the dead end, panic!");
}
fn forever() -> ! {
    loop {
    };
}