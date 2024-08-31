enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 3, y: 4 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 0, 0);

    m.call();
}
