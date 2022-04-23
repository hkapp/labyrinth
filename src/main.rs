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
    let t = Tile {
        left:  Passage::Open,
        right: Passage::Closed,
        up:    Passage::Open,
        down:  Passage::Closed
    };

    print_tile(&t);
}

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
