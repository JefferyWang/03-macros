use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    println!("Up: {:?}", up);
    let left: Direction = 42.into();
    println!("Left: {:?}", left);
}

// impl From<u32> for Direction {
//     fn from(speed: u32) -> Self {
//         Direction::Left(speed)
//     }
// }

// impl From<DirectionUp> for Direction {
//     fn from(up: DirectionUp) -> Self {
//         Direction::Up(up)
//     }
// }
