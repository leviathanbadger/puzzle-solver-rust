#![cfg(test)]

use super::*;

#[test]
fn fitcube_test_against_should_sum_to_27() {
    let result = 1 + TEST_AGAINST.iter().sum::<i32>();
    let expected: i32 = 27;
    assert_eq!(result, expected);
}

const ALL_DIRS: [Dir; 6] = [Dir::Up, Dir::Down, Dir::N, Dir::E, Dir::S, Dir::W];

#[test]
fn dir_get_new_travel_directions_should_not_include_self_or_reverse() {
    for dir in ALL_DIRS {
        for new_dir in dir.get_new_travel_directions() {
            assert_ne!(dir, new_dir);
            assert_ne!(dir.get_reverse(), new_dir);
        }
    }
}

#[test]
fn dir_get_new_travel_directions_should_include_4_options() {
    for dir in ALL_DIRS {
        let opt_count = dir.get_new_travel_directions().count();
        assert_eq!(opt_count, 4);
    }
}

#[test]
fn dir_get_single_travel_amt_should_be_normalized_in_one_direction() {
    for dir in ALL_DIRS {
        let mut travel_dir_count = 0;
        let mut travel_amt_abs = 0;
        let travel_amt = dir.get_single_travel_amt();
        let travel_amt_arr = [travel_amt.0, travel_amt.1, travel_amt.2];
        for q in 0..travel_amt_arr.len() {
            travel_amt_abs += travel_amt_arr[q].abs();
            travel_dir_count += (travel_amt_arr[q] != 0) as i32;
        }
        assert_eq!(travel_dir_count, 1);
        assert_eq!(travel_amt_abs, 1);
    }
}

#[test]
fn op_start_at_apply_should_not_work_if_invalid_position() {
    let mut cube = [[[false; 3]; 3]; 3];
    let mut pos = (0usize, 0usize, 0usize);

    {
        let start_pos = (2usize, 1usize, 0usize);
        let start_op = Op::StartAt(start_pos);
        cube[2][1][0] = true;
        assert_eq!(start_op.apply(&mut cube, &mut pos), false);
        assert_eq!(pos, (0, 0, 0));
    }

    {
        let start_pos = (4usize, 1usize, 0usize);
        let start_op = Op::StartAt(start_pos);
        assert_eq!(start_op.apply(&mut cube, &mut pos), false);
        assert_eq!(pos, (0, 0, 0));
    }
}

#[test]
fn op_start_at_apply_should_be_revsersible() {
    let mut cube = [[[false; 3]; 3]; 3];
    let mut pos = (0usize, 0usize, 0usize);

    let start_pos = (2usize, 1usize, 0usize);
    let start_op = Op::StartAt(start_pos);
    assert!(start_op.apply(&mut cube, &mut pos));

    assert_eq!(cube[2][1][0], true);
    assert_eq!(pos, (2, 1, 0));

    start_op.revert(&mut cube, &mut pos);

    assert_eq!(cube[2][1][0], false);
}

#[test]
fn op_travel_apply_should_not_work_if_invalid_position() {
    let mut cube = [[[false; 3]; 3]; 3];
    cube[0][0][0] = true;
    let mut pos = (0usize, 0usize, 0usize);

    {
        let travel_op = Op::Travel(Dir::Down, 1);
        assert_eq!(travel_op.apply(&mut cube, &mut pos), false);
    }

    {
        let travel_op = Op::Travel(Dir::W, 1);
        assert_eq!(travel_op.apply(&mut cube, &mut pos), false);
    }

    {
        let travel_op = Op::Travel(Dir::Up, 1);
        cube[0][0][1] = true;
        assert_eq!(travel_op.apply(&mut cube, &mut pos), false);
        assert_eq!(pos, (0, 0, 0));
        cube[0][0][1] = false;
    }

    {
        let travel_op = Op::Travel(Dir::Up, 2);
        cube[0][0][2] = true;
        assert_eq!(travel_op.apply(&mut cube, &mut pos), false);
        assert_eq!(pos, (0, 0, 0));
        assert_eq!(cube[0][0][1], false);
    }

    {
        let travel_op = Op::Travel(Dir::E, 3);
        assert_eq!(travel_op.apply(&mut cube, &mut pos), false);
        assert_eq!(pos, (0, 0, 0));
        assert_eq!(cube[1][0][0], false);
        assert_eq!(cube[2][0][0], false);
    }
}

#[test]
fn op_travel_apply_should_be_reversible() {
    let mut cube = [[[false; 3]; 3]; 3];
    cube[2][2][2] = true;
    let mut pos = (2usize, 2usize, 2usize);

    {
        let travel_op = Op::Travel(Dir::Down, 1);
        assert_eq!(travel_op.apply(&mut cube, &mut pos), true);

        assert_eq!(cube[2][2][1], true);
        assert_eq!(pos, (2, 2, 1));
        assert_eq!(cube[2][2][0], false);

        travel_op.revert(&mut cube, &mut pos);

        assert_eq!(cube[2][2][1], false);
        assert_eq!(pos, (2, 2, 2));
        assert_eq!(cube[2][2][2], true);
    }

    {
        let travel_op = Op::Travel(Dir::W, 2);
        assert_eq!(travel_op.apply(&mut cube, &mut pos), true);

        assert_eq!(cube[0][2][2], true);
        assert_eq!(cube[1][2][2], true);
        assert_eq!(pos, (0, 2, 2));

        travel_op.revert(&mut cube, &mut pos);

        assert_eq!(cube[0][2][2], false);
        assert_eq!(cube[1][2][2], false);
        assert_eq!(pos, (2, 2, 2));
        assert_eq!(cube[2][2][2], true);
    }

    {
        let travel_op = Op::Travel(Dir::N, 2);
        let travel_op_2 = Op::Travel(Dir::Down, 1);
        assert_eq!(travel_op.apply(&mut cube, &mut pos), true);
        assert_eq!(travel_op_2.apply(&mut cube, &mut pos), true);

        assert_eq!(cube[2][1][2], true);
        assert_eq!(cube[2][0][2], true);
        assert_eq!(cube[2][0][1], true);
        assert_eq!(pos, (2, 0, 1));

        travel_op_2.revert(&mut cube, &mut pos);
        travel_op.revert(&mut cube, &mut pos);

        assert_eq!(cube[2][1][2], false);
        assert_eq!(cube[2][0][2], false);
        assert_eq!(cube[2][0][1], false);
        assert_eq!(pos, (2, 2, 2));
        assert_eq!(cube[2][2][2], true);
    }
}
