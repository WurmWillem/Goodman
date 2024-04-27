use crate::types::{Kind, Piece, Side};

pub fn make_move(pieces: &mut Vec<Vec<Piece>>, from: (usize, usize), to: (usize, usize)) {
    let mut orig_piece = pieces[from.0][from.1].clone();

    match orig_piece.kind {
        Kind::Pawn(_) => {
            // remove pawn if en passant was done
            if pieces[to.0][to.1].kind == Kind::None {
                pieces[from.0][to.1] = Piece::new_empty();
            }
            // make pawn true if it moved 2 spaces forward
            if (to.0 as i32 - from.0 as i32).abs() == 2 {
                orig_piece.kind = Kind::Pawn(true);
            }
            // make pawn a queen if it crossed the board
            if to.0 == 7 || to.0 == 0 {
                orig_piece = Piece::new(Kind::Queen, orig_piece.side);
            }
        }
        Kind::Rook(_) => orig_piece.kind = Kind::Rook(true),
        Kind::King(_) => orig_piece.kind = Kind::King(true),
        _ => (),
    }

    pieces[to.0][to.1] = orig_piece;
    pieces[from.0][from.1] = Piece::new_empty();
    // pieces[m.0][m.1].selected = false;
}

pub fn calculate_moves(board: &Vec<Vec<Piece>>, j: usize, i: usize) -> Vec<(usize, usize)> {
    let j = j as isize;
    let i = i as isize;
    let moves = match board[j as usize][i as usize].kind {
        Kind::Pawn(_) => generate_pawn_moves(board, i, j),
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
        Kind::Bishop => generate_bishop_moves(board, i, j),
        Kind::Rook(_) => generate_rook_moves(board, i, j),
        Kind::Queen => {
            let mut bishop_moves = generate_bishop_moves(board, i, j);
            bishop_moves.append(&mut generate_rook_moves(board, i, j));
            bishop_moves
        }
        Kind::King(_) => generate_king_moves(board, j, i),
        _ => Vec::new(),
    };
    return_moves_not_on_same_side(board, moves, board[j as usize][i as usize].side)
}

fn generate_king_moves(board: &Vec<Vec<Piece>>, j: isize, i: isize) -> Vec<(usize, usize)> {
    let mut m = return_safe_moves_vec(vec![
        (j, i + 1),
        (j, i - 1),
        (j + 1, i),
        (j - 1, i),
        (j + 1, i + 1),
        (j + 1, i - 1),
        (j - 1, i + 1),
        (j - 1, i - 1),
    ]);
    let j = j as usize;
    let i = i as usize;
    if king(board, j, i) {
        m.push((j, 1));
    }
    m
}

fn king(board: &Vec<Vec<Piece>>, j: usize, i: usize) -> bool {
    if matches!(board[j][i].kind, Kind::King(false)) && matches!(board[j][0].kind, Kind::Rook(false)) {
        for x in i..0 {
            if board[j as usize][x as usize].kind != Kind::None { return false;}
        }
    }
    true
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
    let side = pieces[j as usize][i as usize].side;
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

    let right_up = return_non_blocked_moves(pieces, return_safe_moves_vec(right_up), side);
    let mut right_down = return_non_blocked_moves(pieces, return_safe_moves_vec(right_down), side);
    let mut left_up = return_non_blocked_moves(pieces, return_safe_moves_vec(left_up), side);
    let mut left_down = return_non_blocked_moves(pieces, return_safe_moves_vec(left_down), side);

    let mut vec_all = right_up;
    vec_all.append(&mut right_down);
    vec_all.append(&mut left_up);
    vec_all.append(&mut left_down);

    vec_all
}

fn generate_rook_moves(pieces: &Vec<Vec<Piece>>, i: isize, j: isize) -> Vec<(usize, usize)> {
    let side = pieces[j as usize][i as usize].side;
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

    let vec_right = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_right), side);
    let mut vec_left = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_left), side);
    let mut vec_up = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_up), side);
    let mut vec_down = return_non_blocked_moves(pieces, return_safe_moves_vec(vec_down), side);

    let mut vec_all = vec_right;
    vec_all.append(&mut vec_left);
    vec_all.append(&mut vec_down);
    vec_all.append(&mut vec_up);

    vec_all
}

// returns only the moves that dont hit pieces on the same side, so a white bishop wont hit a white pawn for example
fn return_moves_not_on_same_side(
    pieces: &Vec<Vec<Piece>>,
    moves: Vec<(usize, usize)>,
    piece_side: Side,
) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for m in &moves {
        if pieces[m.0][m.1].side != piece_side {
            vec_safe.push(*m);
        }
    }
    vec_safe
}

// used for bishop, rook and queen, for each diagonal/horizontal
fn return_non_blocked_moves(
    pieces: &Vec<Vec<Piece>>,
    moves: Vec<(usize, usize)>,
    piece_side: Side,
) -> Vec<(usize, usize)> {
    let mut vec_safe: Vec<(usize, usize)> = Vec::new();

    for m in &moves {
        if pieces[m.0][m.1].side == piece_side {
            break;
        } else if pieces[m.0][m.1].side == piece_side.opposite() {
            vec_safe.push(*m);
            break;
        }

        vec_safe.push(*m);
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
