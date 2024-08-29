fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let y = {
        let x = 3;

        x + 1
    };

    println!("The value of y is: {}", y);

    let x = plus_five(5);

    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}
