// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8,u8,u8)
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("hello world".to_string()));
    println!("{:?}", Message::Move{ x:10,y:20});
    println!("{:?}", Message::ChangeColor(255,0,0));
}
