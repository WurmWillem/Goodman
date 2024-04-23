use goodman::prelude::*;

pub fn get_textures(engine: &mut Engine) -> Vec<Texture> {
    let mut textures = vec![];
    create_textures!(engine, textures, 
        "assets/white-pawn.png" "assets/white-knight.png" "assets/white-bishop.png" "assets/white-rook.png" "assets/white-queen.png" "assets/white-king.png" 
        "assets/black-pawn.png" "assets/black-knight.png" "assets/black-bishop.png" "assets/black-rook.png" "assets/black-queen.png" "assets/black-king.png"
        "assets/w.png" "assets/b.png" "assets/yellow.png" "assets/circle.png");
    textures
}
