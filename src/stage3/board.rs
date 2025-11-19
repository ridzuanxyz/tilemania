/// Board management for Stage 3 (15×15 Classic Word Tile Board)

use bevy::prelude::*;
use super::components::PremiumSquare;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// 15×15 word tile game board
#[derive(Resource)]
pub struct Board {
    pub grid: [[Option<char>; 15]; 15],
    pub premium_squares: [[PremiumSquare; 15]; 15],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[None; 15]; 15],
            premium_squares: Self::create_premium_layout(),
        }
    }
}

impl Board {
    /// Create the classic premium square layout
    fn create_premium_layout() -> [[PremiumSquare; 15]; 15] {
        use PremiumSquare::*;

        let mut layout = [[Normal; 15]; 15];

        // Center star (7, 7)
        layout[7][7] = Center;

        // Triple Word (TW) - corners and sides
        let tw_positions = [
            (0, 0), (0, 7), (0, 14),
            (7, 0), (7, 14),
            (14, 0), (14, 7), (14, 14),
        ];
        for &(r, c) in &tw_positions {
            layout[r][c] = TripleWord;
        }

        // Double Word (DW) - diagonal pattern
        let dw_positions = [
            (1, 1), (2, 2), (3, 3), (4, 4),
            (1, 13), (2, 12), (3, 11), (4, 10),
            (13, 1), (12, 2), (11, 3), (10, 4),
            (13, 13), (12, 12), (11, 11), (10, 10),
        ];
        for &(r, c) in &dw_positions {
            layout[r][c] = DoubleWord;
        }

        // Triple Letter (TL)
        let tl_positions = [
            (1, 5), (1, 9),
            (5, 1), (5, 5), (5, 9), (5, 13),
            (9, 1), (9, 5), (9, 9), (9, 13),
            (13, 5), (13, 9),
        ];
        for &(r, c) in &tl_positions {
            layout[r][c] = TripleLetter;
        }

        // Double Letter (DL)
        let dl_positions = [
            (0, 3), (0, 11),
            (2, 6), (2, 8),
            (3, 0), (3, 7), (3, 14),
            (6, 2), (6, 6), (6, 8), (6, 12),
            (7, 3), (7, 11),
            (8, 2), (8, 6), (8, 8), (8, 12),
            (11, 0), (11, 7), (11, 14),
            (12, 6), (12, 8),
            (14, 3), (14, 11),
        ];
        for &(r, c) in &dl_positions {
            layout[r][c] = DoubleLetter;
        }

        layout
    }

    /// Clear the board
    pub fn clear(&mut self) {
        self.grid = [[None; 15]; 15];
    }

    /// Place a letter at position
    pub fn place(&mut self, row: usize, col: usize, letter: char) -> Result<(), String> {
        if row >= 15 || col >= 15 {
            return Err("Position out of bounds".to_string());
        }
        if self.grid[row][col].is_some() {
            return Err("Square already occupied".to_string());
        }
        self.grid[row][col] = Some(letter);
        Ok(())
    }

    /// Get letter at position
    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        if row >= 15 && col >= 15 {
            return None;
        }
        self.grid[row][col]
    }

    /// Check if position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.get(row, col).is_none()
    }

    /// Get premium square at position
    pub fn get_premium(&self, row: usize, col: usize) -> PremiumSquare {
        if row >= 15 || col >= 15 {
            return PremiumSquare::Normal;
        }
        self.premium_squares[row][col]
    }

    /// Check if board is empty (first move)
    pub fn is_board_empty(&self) -> bool {
        for row in &self.grid {
            for &cell in row {
                if cell.is_some() {
                    return false;
                }
            }
        }
        true
    }
}

/// Tile bag for drawing random tiles
#[derive(Resource)]
pub struct TileBag {
    pub tiles: Vec<char>,
    pub remaining_count: usize,
}

impl Default for TileBag {
    fn default() -> Self {
        Self {
            tiles: Self::create_tile_distribution(),
            remaining_count: 100,
        }
    }
}

impl TileBag {
    /// Create standard tile distribution (100 tiles total)
    fn create_tile_distribution() -> Vec<char> {
        let mut tiles = Vec::new();

        // Letter frequencies (standard English word games)
        let distribution = [
            ('A', 9), ('B', 2), ('C', 2), ('D', 4), ('E', 12),
            ('F', 2), ('G', 3), ('H', 2), ('I', 9), ('J', 1),
            ('K', 1), ('L', 4), ('M', 2), ('N', 6), ('O', 8),
            ('P', 2), ('Q', 1), ('R', 6), ('S', 4), ('T', 6),
            ('U', 4), ('V', 2), ('W', 2), ('X', 1), ('Y', 2),
            ('Z', 1), ('_', 2), // 2 blank tiles represented as '_'
        ];

        for (letter, count) in &distribution {
            for _ in 0..*count {
                tiles.push(*letter);
            }
        }

        // Shuffle tiles
        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);

        tiles
    }

    /// Draw N tiles from the bag
    pub fn draw_tiles(&mut self, count: usize) -> Vec<char> {
        let actual_count = count.min(self.tiles.len());
        let drawn: Vec<char> = self.tiles.drain(..actual_count).collect();
        self.remaining_count = self.tiles.len();
        drawn
    }

    /// Draw a single tile
    pub fn draw_one(&mut self) -> Option<char> {
        if self.tiles.is_empty() {
            None
        } else {
            self.remaining_count = self.tiles.len() - 1;
            Some(self.tiles.remove(0))
        }
    }

    /// Return tiles to bag (for exchanges)
    pub fn return_tiles(&mut self, tiles: Vec<char>) {
        self.tiles.extend(tiles);
        let mut rng = thread_rng();
        self.tiles.shuffle(&mut rng);
        self.remaining_count = self.tiles.len();
    }

    /// Reset the bag to full distribution
    pub fn reset(&mut self) {
        self.tiles = Self::create_tile_distribution();
        self.remaining_count = self.tiles.len();
    }

    /// Check if bag is empty
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// Get remaining tile count
    pub fn count(&self) -> usize {
        self.tiles.len()
    }
}

/// Spawn board visually
pub fn spawn_board(
    mut commands: Commands,
    board: Res<Board>,
    asset_server: Res<AssetServer>,
) {
    const SQUARE_SIZE: f32 = 40.0;
    const BOARD_OFFSET_X: f32 = -300.0;
    const BOARD_OFFSET_Y: f32 = 300.0;

    for row in 0..15 {
        for col in 0..15 {
            let x = BOARD_OFFSET_X + (col as f32 * SQUARE_SIZE);
            let y = BOARD_OFFSET_Y - (row as f32 * SQUARE_SIZE);

            let premium = board.get_premium(row, col);
            let color = get_premium_color(premium);

            // Spawn square background
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(SQUARE_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..default()
                },
                super::components::BoardSquare {
                    position: (row, col),
                    premium,
                    occupied_by: None,
                },
            ));

            // Add premium square label
            if premium != PremiumSquare::Normal {
                let label = get_premium_label(premium);
                commands.spawn((
                    Text2d::new(label),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
                    Transform::from_translation(Vec3::new(x, y, 1.0)),
                ));
            }
        }
    }
}

/// Get color for premium square
fn get_premium_color(premium: PremiumSquare) -> Color {
    use PremiumSquare::*;
    match premium {
        Normal => Color::srgb(0.85, 0.85, 0.75),
        DoubleLetter => Color::srgb(0.6, 0.8, 1.0),  // Light blue
        TripleLetter => Color::srgb(0.2, 0.5, 0.9),  // Dark blue
        DoubleWord => Color::srgb(1.0, 0.7, 0.7),    // Light red
        TripleWord => Color::srgb(0.9, 0.2, 0.2),    // Dark red
        Center => Color::srgb(1.0, 0.8, 0.5),        // Gold star
    }
}

/// Get label for premium square
fn get_premium_label(premium: PremiumSquare) -> &'static str {
    use PremiumSquare::*;
    match premium {
        DoubleLetter => "DL",
        TripleLetter => "TL",
        DoubleWord => "DW",
        TripleWord => "TW",
        Center => "★",
        Normal => "",
    }
}
