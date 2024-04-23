use crate::{
    moves::*,
    types::{Kind, Piece, Side, Turn},
    SQUARE,
};
use goodman::prelude::*;

pub struct State {
    turn: Turn,
    pub selected_piece_moves: Vec<(usize, usize)>,
    selected_piece_index: (usize, usize),
}
impl State {
    pub fn new() -> Self {
        Self {
            turn: Turn::White,
            selected_piece_moves: vec![],
            selected_piece_index: (0, 0),
        }
    }
    pub fn check_for_move(&mut self, pieces: &mut Vec<Vec<Piece>>, input: &Input) {
        let clicked_coords = get_clicked_square_coords(&input);

        if let Some(coords) = clicked_coords {
            let (i, j) = coords;

            let mut moved_a_piece = false;
            crate::types::deselect_every_piece(pieces);

            // make move if clicked square is in moves
            if self.selected_piece_moves.len() > 0 {
                let index = self.selected_piece_index;
                let side_clicked = pieces[j][i].side;
                let side_original = pieces[index.0][index.1].side;

                for m in &self.selected_piece_moves {
                    if (j, i) == *m
                        && (side_clicked == Side::opposite(&side_original)
                            || side_clicked == Side::None
                            || side_original == Side::None)
                    {
                        make_move(pieces, index, *m);
                        self.turn = Turn::opposite(&self.turn);
                        self.selected_piece_moves = vec![];
                        moved_a_piece = true;
                        break;
                    }
                }
            }

            // select piece and generate moves for it
            let side = pieces[j][i].side;
            if ((side == Side::White && self.turn == Turn::White)
                || (side == Side::Black && self.turn == Turn::Black))
                && pieces[j][i].kind != Kind::None
                && !moved_a_piece
            {
                pieces[j][i].selected = true;
                // pieces[j][i].moves = calculate_moves(pieces, &pieces[j][i], j, i);
                self.selected_piece_moves = calculate_moves(pieces, &pieces[j][i], j, i);
                self.selected_piece_index = (j, i);
            }

            /*pieces[j][i] = Data::change_value(
                &pieces[j][i],
                Data {
                    index: Data::get_index(&pieces[j][i]),
                    side: Data::get_side(&pieces[j][i]),
                    selected: true,
                    moves: PieceKind::calculate_moves(pieces, &pieces[j][i], j, i),
                    ..Default::default()
                },
            );*/
        }
        // self.selected_piece_moves = vec![];

        /*for j in 0..8 {
            for i in 0..8 {
                if !square_clicked(i, j, input) {
                    continue;
                }

                let mut moved_piece = false;
                crate::types::deselect_every_piece(pieces);

                if moves.len() > 0 {
                    let side_clicked = pieces[j][i].side;
                    let side_original = pieces[index.0][index.1].side;

                    for m in &moves {
                        if (j, i) == *m
                            && (side_clicked == Side::opposite(&side_original)
                                || side_clicked == Side::None
                                || side_original == Side::None)
                        {
                            make_move(pieces, index, *m);
                            self.turn = Turn::opposite(&self.turn);
                            moved_piece = true;
                            break;
                        }
                    }
                }

                let side = pieces[j][i].side;
                if (side == Side::White && self.turn == Turn::Black)
                    || (side == Side::Black && self.turn == Turn::White)
                    || pieces[j][i].kind == Kind::None
                    || moved_piece
                {
                    continue;
                }

                pieces[j][i].selected = true;
                pieces[j][i].moves = calculate_moves(pieces, &pieces[j][i], j, i);
                /*pieces[j][i] = Data::change_value(
                    &pieces[j][i],
                    Data {
                        index: Data::get_index(&pieces[j][i]),
                        side: Data::get_side(&pieces[j][i]),
                        selected: true,
                        moves: PieceKind::calculate_moves(pieces, &pieces[j][i], j, i),
                        ..Default::default()
                    },
                );*/
            }
        }*/
    }

    pub fn render_moves(&self, engine: &mut Engine, textures: &Vec<Texture>) {
        let moves = &self.selected_piece_moves;
        if moves.len() > 0 {
            for m in moves {
                let rect = rect32(
                    m.1 as f32 * SQUARE + SQUARE * 0.345,
                    m.0 as f32 * SQUARE + SQUARE * 0.345,
                    SQUARE * 0.33,
                    SQUARE * 0.33,
                );
                engine.render_texture(rect, &textures[15]);
            }
        }
    }

    /*
    pub fn check_for_move(&mut self, pieces: &mut Vec<Vec<Piece>>, input: &Input) {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let mut index = (0, 0);

        for j in 0..8 {
            for i in 0..8 {
                if pieces[j][i].moves.len() > 0 {
                    moves = pieces[j][i].moves.clone();
                    index = (j, i);
                    break;
                }
            }
        }
        for j in 0..8 {
            for i in 0..8 {
                if !square_clicked(i, j, input) {
                    continue;
                }

                let mut moved_piece = false;
                Kind::deselect_every_piece(pieces);

                if moves.len() > 0 {
                    let side_clicked = pieces[j][i].side;
                    let side_original = pieces[index.0][index.1].side;

                    for m in &moves {
                        if (j, i) == *m
                            && (side_clicked == Side::opposite(&side_original)
                                || side_clicked == Side::None
                                || side_original == Side::None)
                        {
                            Kind::make_move(pieces, index, *m);
                            self.turn = Turn::opposite(&self.turn);
                            moved_piece = true;
                            break;
                        }
                    }
                }

                let side = pieces[j][i].side;
                if (side == Side::White && self.turn == Turn::Black)
                    || (side == Side::Black && self.turn == Turn::White)
                    || pieces[j][i].kind == Kind::None
                    || moved_piece
                {
                    continue;
                }

                pieces[j][i].selected = true;
                pieces[j][i].moves = Kind::calculate_moves(pieces, &pieces[j][i], j, i);
                /*pieces[j][i] = Data::change_value(
                    &pieces[j][i],
                    Data {
                        index: Data::get_index(&pieces[j][i]),
                        side: Data::get_side(&pieces[j][i]),
                        selected: true,
                        moves: PieceKind::calculate_moves(pieces, &pieces[j][i], j, i),
                        ..Default::default()
                    },
                );*/
            }
        }
    }



     */
}

fn square_clicked(x: usize, y: usize, input: &Input) -> bool {
    let x = x as f32;
    let y = y as f32;
    let cursor_pos = vec2(
        input.get_cursor_pos().x as f32,
        input.get_cursor_pos().y as f32,
    );

    if input.is_button_pressed(Button::LeftMouse) {
        // println!("{} {}", cursor_pos.x / SQUARE, cursor_pos.y / SQUARE);
    }

    return input.is_button_pressed(Button::LeftMouse)
        && cursor_pos.x > x * SQUARE
        && cursor_pos.x < x * SQUARE + SQUARE
        && cursor_pos.y > y * SQUARE
        && cursor_pos.y < y * SQUARE + SQUARE;
}

fn get_clicked_square_coords(input: &Input) -> Option<(usize, usize)> {
    if input.is_button_pressed(Button::LeftMouse) {
        let coords = (
            (input.get_cursor_pos().x as f32 / SQUARE) as usize,
            (input.get_cursor_pos().y as f32 / SQUARE) as usize,
        );
        // println!("{:?}", coords);
        return Some(coords);
    }
    None
}
