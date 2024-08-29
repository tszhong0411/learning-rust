fn main() {
    println!("Hello, world!");

    // Mutable variable
    let mut x = 5;

    println!("The value of x is {}", x);

    x = 6;

    println!("The value of x is {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is {}", y);

    let spaces = "    ";
    let spaces = spaces.len(); // shadowing with different type

    println!("{}", spaces);

    // Data types
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{}", guess);

    // | Length | Signed | Unsigned |
    // |--------|--------|----------|
    // | 8-bit  | i8     | u8       |
    // | 16-bit | i16    | u16      |
    // | 32-bit | i32    | u32      |
    // | 64-bit | i64    | u64      |
    // | 128-bit| i128   | u128     |
    // | arch   | isize  | usize    |

    // | Number literals | Example     |
    // |-----------------|-------------|
    // | Decimal         | 98_222      |
    // | Hex             | 0xff        |
    // | Octal           | 0o77        |
    // | Binary          | 0b1111_0000 |
    // | Byte (u8 only)  | b'A'        |

    // Float

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Numeric operations

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    // Boolean

    let t = true;

    let f: bool = false;

    // Character

    let c = 'z';

    let z = 'ℤ';

    let heart_eyed_cat = '😻';

    // Compound types

    // Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // Array

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
}
