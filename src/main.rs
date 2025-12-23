mod board;
use board::piece::Pieces;
fn main() {
    let p = Pieces::Bishop;
    let piece_num: u8 = u8::try_from(p).unwrap();
    let p = Pieces::Bishop;
    println!("{}", piece_num);
    println!("{}", p as u8);
}
