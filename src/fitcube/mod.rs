mod tests;

use std::collections::HashMap;

pub const TEST_AGAINST: [i32; 22] = [2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1];

// cube[x][y][z] - x is east/west, y is north/south, z is up/down
type Cube = [[[bool; 3]; 3]; 3];

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Dir {
    Up,
    Down,
    N,
    E,
    S,
    W
}

fn create_travel_directions(dir: Dir) -> Vec<Dir> {
    let mut vec = vec!();
    if dir != Dir::Up && dir != Dir::Down {
        vec.push(Dir::Up);
        vec.push(Dir::Down);
    }
    if dir != Dir::N && dir != Dir::S {
        vec.push(Dir::N);
        vec.push(Dir::S);
    }
    if dir != Dir::E && dir != Dir::W {
        vec.push(Dir::E);
        vec.push(Dir::W);
    }
    vec
}

lazy_static! {
    static ref TRAVEL_DIRECTIONS: HashMap<Dir, Box<Vec<Dir>>> = {
        let mut map = HashMap::new();
        let all_dirs = [Dir::Up, Dir::Down, Dir::N, Dir::E, Dir::S, Dir::W];
        for dir in all_dirs {
            let vec = Box::new(create_travel_directions(dir));
            map.insert(dir, vec);
        }
        map
    };
}

impl Dir {
    fn get_single_travel_amt(self) -> (i32, i32, i32) {
        match self {
            Dir::Up => (0, 0, 1),
            Dir::Down => (0, 0, -1),
            Dir::N => (0, -1, 0),
            Dir::E => (1, 0, 0),
            Dir::S => (0, 1, 0),
            Dir::W => (-1, 0, 0)
        }
    }

    fn get_new_travel_directions(self) -> impl Iterator<Item = Dir> {
        TRAVEL_DIRECTIONS.get(&self).unwrap().iter().map(|d| *d)
    }

    fn get_reverse(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    StartAt((usize, usize, usize)),
    Travel(Dir, i32)
}

impl Op {
    fn apply(self, cube: &mut Cube, pos: &mut (usize, usize, usize)) -> bool {
        match self {
            Op::StartAt(start_pos) => {
                if start_pos.0 > 2usize || start_pos.1 > 2usize || start_pos.2 > 2usize || cube[start_pos.0][start_pos.1][start_pos.2] {
                    return false;
                }
                cube[start_pos.0][start_pos.1][start_pos.2] = true;
                *pos = start_pos;
            },
            Op::Travel(dir, amt) => {
                let travel_amt = dir.get_single_travel_amt();
                let new_pos_i32 = (pos.0 as i32 + travel_amt.0 * amt, pos.1 as i32 + travel_amt.1 * amt, pos.2 as i32 + travel_amt.2 * amt);
                if new_pos_i32.0 < 0 || new_pos_i32.0 > 2 || new_pos_i32.1 < 0 || new_pos_i32.1 > 2 || new_pos_i32.2 < 0 || new_pos_i32.2 > 2 {
                    return false;
                }
                for q in 0..amt {
                    let new_pos = ((pos.0 as i32 + travel_amt.0 * (q + 1)) as usize, (pos.1 as i32 + travel_amt.1 * (q + 1)) as usize, (pos.2 as i32 + travel_amt.2 * (q + 1)) as usize);
                    if cube[new_pos.0][new_pos.1][new_pos.2] {
                        return false;
                    }
                }
                for q in 0..amt {
                    let new_pos = ((pos.0 as i32 + travel_amt.0 * (q + 1)) as usize, (pos.1 as i32 + travel_amt.1 * (q + 1)) as usize, (pos.2 as i32 + travel_amt.2 * (q + 1)) as usize);
                    cube[new_pos.0][new_pos.1][new_pos.2] = true;
                    if q == amt - 1 {
                        *pos = new_pos;
                    }
                }
            }
        }
        true
    }

    fn revert(self, cube: &mut Cube, pos: &mut (usize, usize, usize)) {
        match self {
            Op::StartAt(start_pos) => {
                cube[start_pos.0][start_pos.1][start_pos.2] = false;
            },
            Op::Travel(dir, amt) => {
                let travel_amt = dir.get_reverse().get_single_travel_amt();
                for q in 0..(amt + 1) {
                    let new_pos = ((pos.0 as i32 + travel_amt.0 * q) as usize, (pos.1 as i32 + travel_amt.1 * q) as usize, (pos.2 as i32 + travel_amt.2 * q) as usize);
                    if q < amt {
                        cube[new_pos.0][new_pos.1][new_pos.2] = false;
                    }
                    else {
                        *pos = new_pos;
                    }
                }
            }
        }
    }
}

type OpStack = Vec<Op>;

fn check_direction(cube: &mut Cube, ops: &mut OpStack, pos: &mut (usize, usize, usize), dir: Dir, test_against_idx: usize) -> bool {
    let travel_op = Op::Travel(dir, TEST_AGAINST[test_against_idx]);
    if !travel_op.apply(cube, pos) {
        return false;
    }

    if test_against_idx == TEST_AGAINST.len() - 1 {
        return true;
    }

    for next_dir in dir.get_new_travel_directions() {
        if check_direction(cube, ops, pos, next_dir, test_against_idx + 1) {
            ops.insert(1, travel_op);
            return true;
        }
    }

    travel_op.revert(cube, pos);
    false
}

pub fn fitcube() -> () {
    let mut cube = [[[false; 3]; 3]; 3];
    let mut pos = (0usize, 0usize, 0usize);
    let mut ops = vec!();

    let start_pos = (0usize, 0usize, 0usize);
    let start_op = Op::StartAt(start_pos);
    start_op.apply(&mut cube, &mut pos);
    ops.push(start_op);

    if !check_direction(&mut cube, &mut ops, &mut pos, Dir::E, 0) {
        println!("Could not find solution.");
    }

    println!("Found solution:\r\n{:?}", ops);
}
