use macros::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction<i32> = DirectionUp::new(42).into();
    println!("Up: {:?}", up);
    let left: Direction<i32> = 42.into();
    println!("Left: {:?}", left);
}

// impl<T> From<DirectionUp<T>> for Direction<T> {
//     fn from(up: DirectionUp<T>) -> Self {
//         Direction::Up(up)
//     }
// }

// impl<T> From<u32> for Direction<T> {
//     fn from(speed: u32) -> Self {
//         Direction::Left(speed)
//     }
// }
