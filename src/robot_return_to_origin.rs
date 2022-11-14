// Use the library provided by the discipline to run parallel functionalities
use diam::join;

// Define the size of the slice to work on, instead of keep dividing
const MIN_SLICE_LENGTH: usize = 10;

// A struct to deal with coordinates in the cartesian plane
struct Coord {
    x: i32,
    y: i32,
}

// Recursive function that returns the displacement vector
fn par_goes_to(moves: &str) -> Coord {
    // Final coordinates
    let mut coord = Coord { x: 0, y: 0 };

    // Base case: if the 'slice' is smaller than MIN_SLICE_LENGTH 
    if moves.len() < MIN_SLICE_LENGTH {
        // Iterate over the chars and increment/decrement the coordinates
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
    // Recursive case: split the string into two parts and sum the coordinates of its results
    else {
        let (left_coord, right_coord) = join(
            || par_goes_to(&moves[..(moves.len() / 2)]),
            || par_goes_to(&moves[(moves.len() / 2)..]),
        );
        coord.x = left_coord.x + right_coord.x;
        coord.y = left_coord.y + right_coord.y;
    }
    // Return the coordinate of the slice analyzed
    coord
}

// Function used to be compatible with the LeetCode structure
pub fn judge_circle(moves: String) -> bool {
    // Convert the String into &str when calling the par_goes_to function
    let coord = par_goes_to(&moves);
    // Check whether the final coordinate is the origin or not
    return coord.x == 0 && coord.y == 0;
}

