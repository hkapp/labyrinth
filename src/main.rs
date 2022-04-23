use rand::Rng;
use std::convert::From;

enum Passage {
    Open,
    Closed
}

struct Tile {
    left:  Passage,
    right: Passage,
    up:    Passage,
    down:  Passage
}

fn main() {
    let mut rng = rand::thread_rng();

    let t = rand_tile(&mut rng);

    print_tile(&t);
}

/**** print_tile() ****/

fn print_tile(tile: &Tile) {
    println!("+{}+", format_horizontal_passage(&tile.up));
    println!("{}   {}",
        format_vertical_passage(&tile.left),
        format_vertical_passage(&tile.right));
    println!("{}   {}",
        format_vertical_passage(&tile.left),
        format_vertical_passage(&tile.right));
    println!("+{}+", format_horizontal_passage(&tile.down));
}

fn format_horizontal_passage(ps: &Passage) -> &str {
    use Passage::*;
    match ps {
        Open   => "   ",
        Closed => "---",
    }
}

fn format_vertical_passage(ps: &Passage) -> char {
    use Passage::*;
    match ps {
        Open   => ' ',
        Closed => '|',
    }
}

/**** rand_tile() ****/

fn rand_tile<R: Rng>(rng: &mut R) -> Tile {
    Tile {
        left:  rng.gen::<bool>().into(),
        right: rng.gen::<bool>().into(),
        up:    rng.gen::<bool>().into(),
        down:  rng.gen::<bool>().into(),
    }
}

impl From<bool> for Passage {
    fn from(b: bool) -> Self {
        match b {
            true  => Passage::Open,
            false => Passage::Closed,
        }
    }
}
