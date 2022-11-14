use diam::join;

const MIN_SLICE_LENGTH: usize = 10;

struct Coord {
    x: i32,
    y: i32,
}

fn par_goes_to(moves: &str) -> Coord {
    let mut coord = Coord { x: 0, y: 0 };

    if moves.len() < MIN_SLICE_LENGTH {
        for c in moves.chars() {
            if c == 'R' {
                coord.x += 1;
            }
            else if c == 'L' {
                coord.x -= 1;
            }
            else if c == 'U' {
                coord.y += 1;
            }
            else if c == 'D' {
                coord.y -= 1;
            }
        }
    }
    else {
        let (left_coord, right_coord) = join(
            || par_goes_to(&moves[..(moves.len() / 2)]),
            || par_goes_to(&moves[(moves.len() / 2)..]),
        );
        coord.x = left_coord.x + right_coord.x;
        coord.y = left_coord.y + right_coord.y;
    }
    coord
}

pub fn judge_circle(moves: String) -> bool {
    let coord = par_goes_to(&moves);
    return coord.x == 0 && coord.y == 0;
}

