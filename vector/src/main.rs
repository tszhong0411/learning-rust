fn main() {
    let mut v = Vec::new();

    // Add elements to the vector
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3, 4];
    let third: &i32 = &v2[2];

    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
        println!("{}", i);
    }
}
