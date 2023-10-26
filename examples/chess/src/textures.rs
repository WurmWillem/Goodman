use goodman::prelude::*;

pub fn get_textures(engine: &mut Engine) -> Vec<Texture> {
    let mut textures = vec![];
    create_textures!(engine, textures, 
        "assets/wpawn.png" "assets/wknight.png" "assets/wbishop.png" "assets/wrook.png" "assets/wqueen.png" "assets/wking.png" 
        "assets/bpawn.png" "assets/bknight.png" "assets/bbishop.png" "assets/brook.png" "assets/bqueen.png" "assets/bking.png"
        "assets/w.png" "assets/b.png");
    textures
}
