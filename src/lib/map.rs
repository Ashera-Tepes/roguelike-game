use rltk::{RandomNumberGenerator, Rltk, RGB};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y * 80 + x) as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // Make the boundary walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // 400 Random walls
    let mut rng = RandomNumberGenerator::new();
    let player_starting_position = xy_idx(40, 25);

    for _i in 0..400 {
        let (x, y) = (rng.roll_dice(1, 79), rng.roll_dice(1, 49));
        let idx = xy_idx(x, y);
        if idx != player_starting_position {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let (mut x, mut y) = (0, 0);
    let grey = RGB::from_f32(0.5, 0.5, 0.5);
    let green = RGB::from_f32(0.0, 1.0, 0.0);
    let black = RGB::from_f32(0., 0., 0.);
    let floor = rltk::to_cp437('.');
    let wall = rltk::to_cp437('#');

    for tile in map.iter() {
        // Rendering a tile depending on tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, grey, black, floor);
            }
            TileType::Wall => {
                ctx.set(x, y, green, black, wall);
            }
        }

        // Move the coordinate
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
