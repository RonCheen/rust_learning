fn variable() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    let _y = 10;
    let (a,b):(bool, bool) = (true, false);
    println!("a={:?}, b={:?}", a,b);

    b = true;
    assert_eq(a, b)
}