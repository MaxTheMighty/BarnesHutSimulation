fn main() {
    let mut list: Vec<i32> = (0..2).collect();
    println!("{:?}",list);
    let i = 1;
    let j = 0;
    let slice = &list[j..i+1];
    println!("{:?}",slice);

}