use anyhow::Result;
use macros::my_vec;

fn main() -> Result<()> {
    let data = my_vec! {1, 2, 3};
    println!("{:?}", data);

    let data = my_vec! {"a", "b", "c"};
    println!("{:?}", data);

    let data = my_vec!["a", "b", "c"];
    println!("{:?}", data);

    let data: Vec<i32> = my_vec![];
    println!("{:?}", data);

    let data = my_vec![1;3];
    println!("{:?}", data);

    let data: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
    ];
    println!("{:?}", data);

    let data = <[_]>::into_vec(Box::new([1, 2, 3, 4]));
    println!("{:?}", data);

    Ok(())
}
