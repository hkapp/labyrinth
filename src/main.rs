use rand::Rng;
use std::convert::From;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

const ROW_NUM: u8 = 5;
const COL_NUM: u8 = ROW_NUM;

fn main() {
    let mut rng = rand::thread_rng();

    for _ in 0..ROW_NUM {
        print_tiles(&rand_row(&mut rng));
    }
}

/**** print_tiles() ****/

fn format_horizontal_passage(ps: Passage) -> &'static str {
    use Passage::*;
    match ps {
        Open   => "   ",
        Closed => "---",
    }
}

fn format_vertical_passage(ps: Passage) -> &'static str {
    use Passage::*;
    match ps {
        Open   => " ",
        Closed => "|",
    }
}

struct RowDisplay {
    top_line: String,
    mid_line: String,
    bot_line: String,
    mid_free: bool
}

fn print_tiles(tiles: &[Tile]) {
    let mut display = RowDisplay::new();

    for t in tiles {
        display.push_tile(&t);
    }

    display.print();
}

impl RowDisplay {
    fn new() -> Self {
        RowDisplay {
            top_line: String::new(),
            mid_line: String::new(),
            bot_line: String::new(),
            mid_free: true,
        }
    }

    fn push_tile(&mut self, t: &Tile) {
        self.push_top(t.up);
        self.push_mid(t.left, t.right);
        self.push_bot(t.down);
    }

    fn push_top(&mut self, ps: Passage) {
        append_horizontal_fmt(&mut self.top_line, ps);
    }

    fn push_bot(&mut self, ps: Passage) {
        append_horizontal_fmt(&mut self.bot_line, ps);
    }

    #[allow(unused_parens)]
    fn push_mid(&mut self, left: Passage, right: Passage) {
        if self.mid_free {
            append_const(&mut self.mid_line, format_vertical_passage(left));
        }
        append_const(&mut self.mid_line, "   ");

        if right == Passage::Closed {
            append_const(&mut self.mid_line, format_vertical_passage(right));
        }
        self.mid_free = (right == Passage::Open);
    }

    fn print(&self) {
        println!("{}", self.top_line);
        println!("{}", self.mid_line);
        println!("{}", self.mid_line);
        println!("{}", self.bot_line);
    }
}

fn append_horizontal_fmt(buf: &mut String, ps: Passage) {
    if buf.is_empty() {
        append_corner(buf);
    }
    append_const(buf, format_horizontal_passage(ps));
    append_corner(buf);
}

fn append_const(buf: &mut String, suffix: &str) {
    buf.push_str(suffix);
}

fn append_corner(buf: &mut String) {
    append_const(buf, "+");
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

fn rand_row<R: Rng>(rng: &mut R) -> Vec<Tile> {
    let row: Vec<Tile> =
        (0..COL_NUM).into_iter()
            .map(|_| rand_tile(rng))
            .collect();

    return row;
}
