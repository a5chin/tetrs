mod blocks;

fn main() {
    let mc: blocks::mino::Controller = blocks::mino::new(blocks::mino::Kind::J);
    println!("{:?}", mc)
}
