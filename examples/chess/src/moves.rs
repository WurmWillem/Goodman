use crate::types::{Kind, Piece, Side};

pub fn make_move(pieces: &mut Vec<Vec<Piece>>, index: (usize, usize), m: (usize, usize)) {
    let mut orig_piece = pieces[index.0][index.1].clone();

    // remove pawn if en passant was done
    if matches!(orig_piece.kind, Kind::Pawn(_)) && m.1 != 0 {
        pieces[index.0][m.1] = Piece::new_empty();
    }

    // make pawn true if it moved 2 spaces forward
    if matches!(orig_piece.kind, Kind::Pawn(_)) && (m.0 as i32 - index.0 as i32).abs() == 2 {
        orig_piece.kind = Kind::Pawn(true);
    }

    pieces[m.0][m.1] = orig_piece;
    pieces[index.0][index.1] = Piece::new_empty();
    // pieces[m.0][m.1].selected = false;
}

pub fn calculate_moves(
    pieces: &Vec<Vec<Piece>>,
    piece: &Piece,
    j: usize,
    i: usize,
) -> Vec<(usize, usize)> {
    let j = j as isize;
    let i = i as isize;
    match piece.kind {
        Kind::Pawn(_) => generate_pawn_moves(pieces, i, j),
        Kind::Knight => return_safe_moves_vec(vec![
            (j - 2, i + 1),
            (j - 2, i - 1),
            (j + 2, i + 1),
            (j + 2, i - 1),
            (j - 1, i - 2),
            (j - 1, i + 2),
            (j + 1, i - 2),
            (j + 1, i + 2),
        ]),
        Kind::Bishop => generate_bishop_moves(pieces, i, j),
        Kind::Rook => generate_rook_moves(pieces, i, j),
        Kind::Queen => {
            let mut bishop_moves = generate_bishop_moves(pieces, i, j);
            let mut rook_moves = generate_rook_moves(pieces, i, j);

            bishop_moves.append(&mut rook_moves);
            bishop_moves
        }
        Kind::King => return_safe_moves_vec(vec![
            (j, i + 1),
            (j, i - 1),
            (j + 1, i),
            (j - 1, i),
            (j + 1, i + 1),
            (j + 1, i - 1),
            (j - 1, i + 1),
            (j - 1, i - 1),
        ]),
        _ => Vec::new(),
    }
}

fn generate_pawn_moves(pieces: &Vec<Vec<Piece>>, i: isize, j: isize) -> Vec<(usize, usize)> {
    let mut moves: Vec<(usize, usize)> = Vec::new();

    if pieces[j as usize][i as usize].side == Side::White {
        let (safe, en_pass) = return_if_safe(j - 1, i - 1);
        if safe
            && matches!(pieces[j as usize][en_pass.1].kind, Kind::Pawn(true))
            && pieces[en_pass.0][en_pass.1].kind == Kind::None
            && pieces[j as usize][en_pass.1].side == Side::Black
        {
            moves.append(&mut vec![(en_pass.0, en_pass.1)])
        }
        let (safe, en_pass) = return_if_safe(j - 1, i + 1);
        if safe
            && matches!(pieces[j as usize][en_pass.1].kind, Kind::Pawn(true))
            && pieces[en_pass.0][en_pass.1].kind == Kind::None
            && pieces[j as usize][en_pass.1].side == Side::Black
        {
            moves.append(&mut vec![(en_pass.0, en_pass.1)])
        }

        let (safe, forward) = return_if_safe(j - 1, i);
        if safe {
            if pieces[forward.0][forward.1].kind == Kind::None {
                moves.append(&mut return_safe_moves_vec(vec![(j - 1, i)]));
            }
        }
        let (safe, forward) = return_if_safe(j - 2, i);
        if safe && j == 6 {
            if pieces[forward.0][forward.1].kind == Kind::None {
                moves.append(&mut return_safe_moves_vec(vec![(j - 2, i)]));
            }
        }
        let (safe, right_forward) = return_if_safe(j - 1, i + 1);
        if safe {
            if pieces[right_forward.0][right_forward.1].side == Side::Black {
                moves.append(&mut return_safe_moves_vec(vec![(j - 1, i + 1)]));
            }
        }
        let (safe, left_forward) = return_if_safe(j - 1, i - 1);
        if safe {
            if pieces[left_forward.0][left_forward.1].side == Side::Black {
                moves.append(&mut return_safe_moves_vec(vec![(j - 1, i - 1)]));
            }
        }
    } else if pieces[j as usize][i as usize].side == Side::Black {
        let (safe, en_pass) = return_if_safe(j + 1, i - 1);
        if safe
            && matches!(pieces[j as usize][en_pass.1].kind, Kind::Pawn(true))
            && pieces[en_pass.0][en_pass.1].kind == Kind::None
            && pieces[j as usize][en_pass.1].side == Side::White
        {
            moves.append(&mut vec![(en_pass.0, en_pass.1)])
        }
        let (safe, en_pass) = return_if_safe(j + 1, i + 1);
        if safe
            && matches!(pieces[j as usize][en_pass.1].kind, Kind::Pawn(true))
            && pieces[en_pass.0][en_pass.1].kind == Kind::None
            && pieces[j as usize][en_pass.1].side == Side::White
        {
            moves.append(&mut vec![(en_pass.0, en_pass.1)])
        }

        let (safe, forward) = return_if_safe(j + 1, i);
        if safe {
            if pieces[forward.0][forward.1].kind == Kind::None {
                moves.append(&mut return_safe_moves_vec(vec![(j + 1, i)]));
            }
        }
        let (safe, forward) = return_if_safe(j + 2, i);
        if safe && j == 1 {
            if pieces[forward.0][forward.1].kind == Kind::None {
                moves.append(&mut return_safe_moves_vec(vec![(j + 2, i)]));
            }
        }
        let (safe, right_forward) = return_if_safe(j + 1, i + 1);
        if safe {
            if pieces[right_forward.0][right_forward.1].side == Side::White {
                moves.append(&mut return_safe_moves_vec(vec![(j + 1, i + 1)]));
            }
        }
        let (safe, left_forward) = return_if_safe(j + 1, i - 1);
        if safe {
            if pieces[left_forward.0][left_forward.1].side == Side::White {
                moves.append(&mut return_safe_moves_vec(vec![(j + 1, i - 1)]));
            }
        }
    } else {
        panic!("side is unknown");
    }
    moves
}

fn generate_bishop_moves(pieces: &Vec<Vec<Piece>>, i: isize, j: isize) -> Vec<(usize, usize)> {
    let mut right_up: Vec<(isize, isize)> = Vec::new();
    let mut left_up: Vec<(isize, isize)> = Vec::new();
    let mut left_down: Vec<(isize, isize)> = Vec::new();
    let mut right_down: Vec<(isize, isize)> = Vec::new();

    let mut x = 1;
    for _ in (i + 1)..8 {
        right_up.push((j + x, i + x));
        right_down.push((j - x, i + x));
        x += 1;
    }

    let mut x = 1;
    for _ in 1..(i + 1) {
        left_up.push((j + x, i - x));
        left_down.push((j - x, i - x));
        x += 1;
    }

    let mut right_up = return_non_blocked_moves(pieces, return_safe_moves_vec(right_up));
    let mut right_down = return_non_blocked_moves(pieces, return_safe_moves_vec(right_down));
    let mut left_up = return_non_blocked_moves(pieces, return_safe_moves_vec(left_up));
    let mut left_down = return_non_blocked_moves(pieces, return_safe_moves_vec(left_down));

    right_up.append(&mut left_up);
    right_down.append(&mut left_down);
    right_up.append(&mut right_down);

    let vec_all = right_up.clone();
    vec_all
}

fn generate_rook_moves(pieces: &Vec<Vec<Piece>>, i: isize, j: isize) -> Vec<(usize, usize)> {
    let mut vec_right: Vec<(isize, isize)> = Vec::new();
    let mut vec_left: Vec<(isize, isize)> = Vec::new();
    let mut vec_up: Vec<(isize, isize)> = Vec::new();
    let mut vec_down: Vec<(isize, isize)> = Vec::new();

    let mut x = 1;
    for _ in (i + 1)..8 {
        vec_right.push((j, i + x));
        x += 1;
    }

    let mut x = 1;
    for _ in 1..(i + 1) {
        vec_left.push((j, i - x));
        x += 1;
    }

    let mut x = 1;
    for _ in (j + 1)..8 {
        vec_up.push((j + x, i));
        x += 1;
    }

    let mut x = 1;
    for _ in 1..(j + 1) {
        vec_down.push((j - x, i));
        x += 1;
    }

    let mut vec_right = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_right));
    let mut vec_left = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_left));
    let mut vec_up = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_up));
    let mut vec_down = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_down));

    vec_right.append(&mut vec_left);
    vec_up.append(&mut vec_down);
    vec_right.append(&mut vec_up);

    let vec_all = vec_right.clone();
    vec_all
}

fn return_non_blocked_moves(
    pieces: &Vec<Vec<Piece>>,
    vec: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for v in &vec {
        if pieces[v.0][v.1].kind != Kind::None {
            vec_safe.push(*v);
            break;
        }
        vec_safe.push(*v);
    }
    vec_safe
}

fn return_if_safe(x: isize, y: isize) -> (bool, (usize, usize)) {
    if x >= 0 && x < 8 && y >= 0 && y < 8 {
        return (true, (x as usize, y as usize));
    }
    (false, (99, 99))
}

fn return_safe_moves_vec(vec: Vec<(isize, isize)>) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for v in &vec {
        if v.0 >= 0 && v.0 < 8 && v.1 >= 0 && v.1 < 8 {
            vec_safe.push((v.0 as usize, v.1 as usize));
        }
    }
    vec_safe
}
