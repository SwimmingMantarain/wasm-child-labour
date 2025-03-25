use macroquad::prelude::*;
use crate::rand::gen_range;

const PLAYER_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const PLAYER_SPEED: f32 = 200.0;

struct Player {
    pos: Vec2,
    vel: Vec2,
    on_ground: bool,
    jump_requested: bool,
}

impl Player {
    fn new() -> Self {
        Self {
            pos: Vec2::new(screen_width() / 2.0, screen_height() - PLAYER_SIZE.y),
            vel: Vec2::ZERO,
            on_ground: false,
            jump_requested: false,
        }
    }

    fn update(&mut self, dt: f32, touch_x: Option<f32>) {
        let mut dx = 0.0;
        if is_key_down(KeyCode::Left) { dx -= 1.0; }
        if is_key_down(KeyCode::Right) { dx += 1.0; }

        // Handle touch input for horizontal movement
        if let Some(x) = touch_x {
            dx = if x < screen_width() / 2.0 { -1.0 } else { 1.0 };
        }

        self.vel.x = dx * PLAYER_SPEED;

        if self.on_ground && (is_key_pressed(KeyCode::Space) || self.jump_requested) {
            self.on_ground = false;
            self.jump_requested = false;
        }

        self.pos += self.vel * dt;

        if self.pos.y > screen_height() - PLAYER_SIZE.y {
            self.pos.y = screen_height() - PLAYER_SIZE.y;
            self.vel.y = 0.0;
            self.on_ground = true;
        }

        self.pos.x = self.pos.x.clamp(0.0, screen_width() - PLAYER_SIZE.x);
    }

    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, PLAYER_SIZE.x, PLAYER_SIZE.y, BLUE);
    }
}

enum TileType {
    Empty,
    Dirt,
    Bedrock,
    Chest,
    IronOre,
    GoldOre,
    DiamondOre,
    CoalOre,
}
struct Tile {
    typ: TileType,
    hidden: bool,
    is_unbreakable: bool,
    cp_pos: Option<Vec2>,
}

struct TileGrid {
    tiles: Vec<Vec<Tile>>,
    pos: Vec2,
    tile_size: f32,
}

impl TileGrid {
    fn new() -> Self {
        Self {
            tiles: Vec::new(),
            pos: Vec2::ZERO,
            tile_size: 24.0,
        }
    }

    fn init(&mut self) {
        let tiles = self.gen_tile_grid();
        let tile_size: f32 = screen_width() / 20.0;

        self.tiles = tiles;
        self.tile_size = tile_size;
    }

    fn gen_tile_grid(&self) -> Vec<Vec<Tile>> {
        let mut tiles = Vec::new();
        // Initial starting platform
        let mut row = Vec::new();
        for _ in 0..24 {
            row.push(Tile {
                typ: TileType::Dirt,
                hidden: false,
                is_unbreakable: true,
                cp_pos: None,
            });
        }
        tiles.push(row);

        for _ in 0..23 {
            let mut row = Vec::new();
            for _ in 0..24 {
                row.push(Tile {
                    typ: TileType::Dirt,
                    hidden: false,
                    is_unbreakable: true,
                    cp_pos: None,
                });
            }
            tiles.push(row);
        }

        tiles
    }

    fn draw(&self) {
        let mut pos = Vec2::ZERO;
        for row in &self.tiles {
            for tile in row {
                // Choose randomly between a few different colors
                let index = gen_range(0, 3);
                let color = match index {
                    0 => RED,
                    1 => GREEN,
                    2 => BLUE,
                    3 => YELLOW,
                    _ => BLACK,
                };
                if !tile.hidden {
                    draw_rectangle(pos.x, pos.y, self.tile_size, self.tile_size, color);
                    pos.x += self.tile_size;
                }
            }
            pos.x = 0.0;
            pos.y += self.tile_size;
        }
    }
}

#[macroquad::main("Platformer")]
async fn main() {
    let mut player = Player::new();
    let mut tile_grid = TileGrid::new();
    tile_grid.init();

    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        let mut touch_x = None;
        let mut jump_requested = false;

        // Handle touch input
        if let Some(touch) = touches().get(0) {
            touch_x = Some(touch.position.x);
            if touch.phase == TouchPhase::Started {
                jump_requested = true;
            }
        }

        tile_grid.draw();

        player.jump_requested = jump_requested;
        player.update(dt, touch_x);
        player.draw();

        draw_text("Use arrow keys or touch sides to move, space or tap to jump", 10.0, 20.0, 20.0, BLACK);

        next_frame().await
    }
}