fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}
