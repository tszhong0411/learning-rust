use rand::Rng;
use std::{collections::HashMap, fmt, io};

fn f1() -> fmt::Result {
    Ok(())
}

fn f2() -> io::Result<()> {
    Ok(())
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
