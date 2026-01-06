#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize( i32,i32),
    Move(i32,i32),
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit
}

fn main() {
    println!("{:?}", Message::Resize(5,3));
    println!("{:?}", Message::Move(3,4));
    println!("{:?}", Message::Echo(String::from("echo")));
    println!("{:?}", Message::ChangeColor(0,0,0));
    println!("{:?}", Message::Quit);
}
