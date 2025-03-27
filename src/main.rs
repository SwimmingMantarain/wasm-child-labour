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
            tile_size: screen_width() / 20.0,
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
                typ: TileType::Bedrock,
                hidden: false,
                is_unbreakable: true,
            });
        }
        tiles.push(row);

        // Left wall
        let mut row = Vec::new();

        row.push(Tile {
            typ: TileType::Bedrock,
            hidden: false,
            is_unbreakable: true,
        });

        // Starting Hallway
        for _ in 0..18 {
            row.push(Tile {
                typ: TileType::Empty,
                hidden: false,
                is_unbreakable: true,
            });
        }

        // Right wall
        row.push(Tile {
            typ: TileType::Bedrock,
            hidden: false,
            is_unbreakable: true,
        });

        tiles.push(row);

        // Everything after
        for _ in 0..14 {
            let mut row = Vec::new();

            // Left wall
            row.push(Tile {
                typ: TileType::Bedrock,
                hidden: false,
                is_unbreakable: true,
            });

            // Rest of the wall
            for _ in 0..18 {
                let random_val = gen_range(0, 100);
                let tile: Tile;
                match random_val {
                    // Dirt
                    0..=69 => tile = Tile {
                        typ: TileType::Dirt,
                        hidden: false,
                        is_unbreakable: false,
                    },
                    // Coal Ore
                    70..=83 => tile = Tile {
                        typ: TileType::CoalOre,
                        hidden: false,
                        is_unbreakable: false,
                    },
                    // Iron Ore
                    84..=94 => tile = Tile {
                        typ: TileType::IronOre,
                        hidden: false,
                        is_unbreakable: false,
                    },
                    // Gold Ore
                    95..=98 => tile = Tile {
                        typ: TileType::GoldOre,
                        hidden: false,
                        is_unbreakable: false,
                    },
                    // Diamond Ore
                    99 => tile = Tile {
                        typ: TileType::DiamondOre,
                        hidden: false,
                        is_unbreakable: false,
                    },
                    // Chest
                    100 => tile = Tile {
                        typ: TileType::Chest,
                        hidden: false,
                        is_unbreakable: false,
                    },
                    // Bedrock - never appears in normal generation (kept as failsafe)
                    _ => tile = Tile {
                        typ: TileType::Bedrock,
                        hidden: false,
                        is_unbreakable: false,
                    },
                }
                row.push(tile);
            }

            // Right wall
            row.push(Tile {
                typ: TileType::Bedrock,
                hidden: false,
                is_unbreakable: true,
            });

            tiles.push(row);
        }

        tiles
    }

    fn move_up(&mut self) {
        self.pos.y += self.tile_size / 10.;
        if self.pos.y >= 0.0 {
            self.pos.y = 0.0;
        }
    }

    fn move_down(&mut self) {
        self.pos.y -= self.tile_size / 10.;
    }

    fn draw(&self, textures: &Textures) {
        let mut pos = self.pos;
        for row in &self.tiles {
            for tile in row {
                // Choose randomly between a few different colors
                let mut color = None;
                let mut texture = None;

                match tile.typ {
                    TileType::Empty => texture = Some(&textures.empty_dirt),
                    TileType::Dirt => texture = Some(&textures.dirt),
                    TileType::Bedrock => texture = Some(&textures.bedrock),
                    TileType::Chest => texture = Some(&textures.chest),
                    TileType::IronOre => texture = Some(&textures.iron_ore),
                    TileType::GoldOre => texture = Some(&textures.gold_ore),
                    TileType::DiamondOre => texture = Some(&textures.diamond_ore),
                    TileType::CoalOre => texture = Some(&textures.coal_ore),
                }

                if !tile.hidden {
                    if let Some(color) = color {
                        draw_rectangle(pos.x, pos.y, self.tile_size, self.tile_size, color);
                    } else {
                        if let Some(texture) = texture {
                            draw_texture_ex(texture, pos.x, pos.y, WHITE, DrawTextureParams {
                                dest_size: Some(Vec2::new(self.tile_size, self.tile_size)),
                                ..Default::default()
                            });
                        }
                        
                    }
                    draw_rectangle_lines(pos.x, pos.y, self.tile_size, self.tile_size, 2.5, BLACK);
                    pos.x += self.tile_size;
                }
            }
            pos.x = 0.0;
            pos.y += self.tile_size;
        }
    }
}

struct Textures {
    empty_dirt: Texture2D,
    dirt: Texture2D,
    bedrock: Texture2D,
    chest: Texture2D,
    iron_ore: Texture2D,
    gold_ore: Texture2D,
    diamond_ore: Texture2D,
    coal_ore: Texture2D,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Child Labour: Epilepsy Edition FHD".to_owned(),
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load textures
    let textures = Textures {
        empty_dirt: load_texture("assets/dirt_empty.png").await.unwrap(),
        dirt: load_texture("assets/dirt.png").await.unwrap(),
        bedrock: load_texture("assets/bedrock.png").await.unwrap(),
        chest: load_texture("assets/chest.png").await.unwrap(),
        iron_ore: load_texture("assets/iron.png").await.unwrap(),
        gold_ore: load_texture("assets/gold.png").await.unwrap(),
        diamond_ore: load_texture("assets/diamond.png").await.unwrap(),
        coal_ore: load_texture("assets/coal.png").await.unwrap(),
    };

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

        if is_key_down(KeyCode::Up) {
            tile_grid.move_up();
        };
        if is_key_down(KeyCode::Down) {
            tile_grid.move_down();
        };

        tile_grid.tile_size = screen_width() / 20.0;
        tile_grid.draw(&textures);

        player.jump_requested = jump_requested;
        player.update(dt, touch_x);
        player.draw();

        next_frame().await
    }
}