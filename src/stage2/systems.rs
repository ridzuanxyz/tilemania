/// Systems for Stage 2 gameplay logic

use bevy::prelude::*;
use rand::Rng;
use super::components::*;
use super::{Stage2Config, Stage2State};
use crate::plugins::state::GameState;
use crate::lexicon::Lexicon;
use crate::scoring::ScoreCalculator;

const TILE_SIZE: f32 = 64.0;
const GRID_SPACING: f32 = 8.0;

/// Spawns the 8x8 grid when gameplay starts
pub fn spawn_grid(
    mut commands: Commands,
    config: Res<Stage2Config>,
    asset_server: Res<AssetServer>,
) {
    let grid_size = config.grid_size;
    let total_width = (grid_size as f32) * (TILE_SIZE + GRID_SPACING);
    let start_x = -total_width / 2.0 + TILE_SIZE / 2.0;
    let start_y = -total_width / 2.0 + TILE_SIZE / 2.0;

    // Spawn grid container
    commands.spawn((
        GameGrid { size: grid_size },
        SpatialBundle::default(),
    ));

    // Spawn 8x8 grid of tiles with staggered animation
    for row in 0..grid_size {
        for col in 0..grid_size {
            let letter = get_weighted_random_letter();
            let x = start_x + (col as f32) * (TILE_SIZE + GRID_SPACING);
            let y = start_y + (row as f32) * (TILE_SIZE + GRID_SPACING);

            // Stagger spawn animation based on position (diagonal wave)
            let tile_index = row + col;
            let stagger_delay = tile_index as f32 * 0.02; // 20ms delay per diagonal

            // Spawn tile with letter text as child
            commands.spawn((
                GridTile {
                    letter,
                    grid_pos: (row, col),
                    is_selected: false,
                    is_matched: false,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.85, 0.85, 0.95),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                SpawnAnimation {
                    elapsed: -stagger_delay, // Negative elapsed = delayed start
                    duration: 0.3, // 300ms bounce-in
                },
            )).with_children(|parent| {
                // Spawn letter text as child so it inherits transformations
                parent.spawn((
                    Text2d::new(letter.to_string()),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 42.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.1, 0.1, 0.1)),
                    Transform::from_xyz(0.0, 0.0, 1.0), // Relative to parent
                ));
            });
        }
    }

    info!("Spawned {}x{} grid", grid_size, grid_size);
}

/// Detects which tile the mouse is hovering over
pub fn detect_tile_hover(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    tile_query: Query<(Entity, &Transform), With<GridTile>>,
    hovered_query: Query<Entity, With<HoveredTile>>,
) {
    // Get mouse position in world coordinates
    let Ok(window) = windows.get_single() else { return; };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };

    let Some(cursor_pos) = window.cursor_position() else {
        // No cursor position - remove all hover markers
        for entity in hovered_query.iter() {
            commands.entity(entity).remove::<HoveredTile>();
        }
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };

    // Find closest tile under cursor
    let mut closest_tile: Option<(Entity, f32)> = None;

    for (entity, transform) in tile_query.iter() {
        let tile_pos = transform.translation.truncate();
        let distance = tile_pos.distance(world_pos);

        if distance < TILE_SIZE / 2.0 {
            match closest_tile {
                None => closest_tile = Some((entity, distance)),
                Some((_, prev_distance)) if distance < prev_distance => {
                    closest_tile = Some((entity, distance));
                }
                _ => {}
            }
        }
    }

    // Update hover markers
    match closest_tile {
        Some((hovered_entity, _)) => {
            // Remove hover from all other tiles
            for entity in hovered_query.iter() {
                if entity != hovered_entity {
                    commands.entity(entity).remove::<HoveredTile>();
                }
            }
            // Add hover to the closest tile
            commands.entity(hovered_entity).insert(HoveredTile);
        }
        None => {
            // No tile under cursor - remove all hover markers
            for entity in hovered_query.iter() {
                commands.entity(entity).remove::<HoveredTile>();
            }
        }
    }
}

/// Handles tile selection via mouse click
pub fn handle_tile_selection(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut tile_query: Query<(Entity, &mut GridTile, &Transform)>,
    mut state: ResMut<Stage2State>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Get selected tile's grid position before iterating (to avoid borrow conflicts)
            let selected_grid_pos = if let Some(selected_entity) = state.selected_tile {
                tile_query.get(selected_entity).ok().map(|(_, tile, _)| tile.grid_pos)
            } else {
                None
            };

            // Check if click hit a tile
            for (entity, mut tile, transform) in tile_query.iter_mut() {
                let tile_pos = transform.translation.truncate();
                let distance = tile_pos.distance(world_pos);

                if distance < TILE_SIZE / 2.0 && !tile.is_matched {
                    if let Some(selected) = state.selected_tile {
                        // Second tile selected - check if adjacent
                        if selected != entity {
                            if let Some((r1, c1)) = selected_grid_pos {
                                let (r2, c2) = tile.grid_pos;

                                // Check if adjacent (not diagonal)
                                let is_adjacent = (r1 == r2 && c1.abs_diff(c2) == 1)
                                    || (c1 == c2 && r1.abs_diff(r2) == 1);

                                if is_adjacent {
                                    // Trigger swap
                                    info!("Swapping tiles at ({},{}) and ({},{})", r1, c1, r2, c2);
                                    // Swap will be handled in separate system
                                    state.selected_tile = Some(entity); // Store second tile for swap
                                } else {
                                    // Not adjacent - select new tile
                                    commands.entity(selected).remove::<SelectedTile>();
                                    tile.is_selected = true;
                                    commands.entity(entity).insert(SelectedTile);
                                    state.selected_tile = Some(entity);
                                }
                            }
                        }
                    } else {
                        // First tile selected
                        tile.is_selected = true;
                        commands.entity(entity).insert(SelectedTile);
                        state.selected_tile = Some(entity);
                        info!("Selected tile at ({},{})", tile.grid_pos.0, tile.grid_pos.1);
                    }
                    break;
                }
            }
        }
    }
}

/// Handles tile swapping animation and logic
pub fn handle_tile_swap(
    mut commands: Commands,
    mut tile_query: Query<(Entity, &mut GridTile, &mut Transform)>,
    mut state: ResMut<Stage2State>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // This system would handle the actual swap logic
    // For now, just a placeholder showing the concept

    // When two adjacent tiles are selected, swap their letters and positions
    // Then check for word matches
}

/// Finds word matches (horizontal and vertical 3-4 letter words)
pub fn find_word_matches(
    mut tile_query: Query<(Entity, &mut GridTile)>,
    config: Res<Stage2Config>,
    mut state: ResMut<Stage2State>,
) {
    let grid_size = config.grid_size;

    // Build grid representation
    let mut grid: Vec<Vec<Option<(Entity, char)>>> = vec![vec![None; grid_size]; grid_size];

    for (entity, tile) in tile_query.iter() {
        let (row, col) = tile.grid_pos;
        grid[row][col] = Some((entity, tile.letter));
    }

    let mut matched_entities = Vec::new();
    let mut words_found = Vec::new();

    // Check horizontal words (3-4 letters)
    for row in 0..grid_size {
        for col in 0..grid_size {
            // Try 4-letter words first
            if col + 4 <= grid_size {
                let mut word = String::new();
                let mut entities = Vec::new();

                for i in 0..4 {
                    if let Some((entity, letter)) = grid[row][col + i] {
                        word.push(letter);
                        entities.push(entity);
                    }
                }

                if word.len() == 4 && config.four_letter_words.contains(&word.to_uppercase()) {
                    info!("Found 4-letter word: {}", word);
                    matched_entities.extend(entities);
                    words_found.push(word.to_uppercase());
                }
            }

            // Try 3-letter words
            if col + 3 <= grid_size {
                let mut word = String::new();
                let mut entities = Vec::new();

                for i in 0..3 {
                    if let Some((entity, letter)) = grid[row][col + i] {
                        word.push(letter);
                        entities.push(entity);
                    }
                }

                if word.len() == 3 && config.three_letter_words.contains(&word.to_uppercase()) {
                    info!("Found 3-letter word: {}", word);
                    matched_entities.extend(entities);
                    words_found.push(word.to_uppercase());
                }
            }
        }
    }

    // Check vertical words (3-4 letters)
    for col in 0..grid_size {
        for row in 0..grid_size {
            // Try 4-letter words
            if row + 4 <= grid_size {
                let mut word = String::new();
                let mut entities = Vec::new();

                for i in 0..4 {
                    if let Some((entity, letter)) = grid[row + i][col] {
                        word.push(letter);
                        entities.push(entity);
                    }
                }

                if word.len() == 4 && config.four_letter_words.contains(&word.to_uppercase()) {
                    info!("Found 4-letter word (vertical): {}", word);
                    matched_entities.extend(entities);
                    words_found.push(word.to_uppercase());
                }
            }

            // Try 3-letter words
            if row + 3 <= grid_size {
                let mut word = String::new();
                let mut entities = Vec::new();

                for i in 0..3 {
                    if let Some((entity, letter)) = grid[row + i][col] {
                        word.push(letter);
                        entities.push(entity);
                    }
                }

                if word.len() == 3 && config.three_letter_words.contains(&word.to_uppercase()) {
                    info!("Found 3-letter word (vertical): {}", word);
                    matched_entities.extend(entities);
                    words_found.push(word.to_uppercase());
                }
            }
        }
    }

    // Mark matched tiles
    if !matched_entities.is_empty() {
        for entity in matched_entities {
            if let Ok((_, mut tile)) = tile_query.get_mut(entity) {
                tile.is_matched = true;
            }
        }

        // Update state
        state.words_found.extend(words_found.clone());
        state.combo_count += 1;

        // Calculate score
        let total_score: u32 = words_found.iter()
            .map(|word| {
                let calculator = ScoreCalculator::new();
                calculator.calculate_score(word, 1.0, state.combo_count as f32)
            })
            .sum();

        state.score += total_score;
        info!("Found {} words for {} points (Combo: {}x)",
            words_found.len(), total_score, state.combo_count);
    }
}

/// Clears matched tiles with animation
pub fn clear_matched_words(
    mut commands: Commands,
    mut tile_query: Query<(Entity, &GridTile, &Transform), With<GridTile>>,
    time: Res<Time>,
) {
    for (entity, tile, transform) in tile_query.iter() {
        if tile.is_matched {
            // Add match animation component
            commands.entity(entity).insert(MatchedTile {
                elapsed: 0.0,
                duration: 0.5, // 0.5 second animation
            });

            // Particle effect at tile position
            // (Placeholder - actual implementation would spawn particles)

            info!("Clearing matched tile at ({},{}) with letter {}",
                tile.grid_pos.0, tile.grid_pos.1, tile.letter);
        }
    }
}

/// Despawn tiles after their match animation completes
pub fn despawn_matched_tiles(
    mut commands: Commands,
    time: Res<Time>,
    mut matched_query: Query<(Entity, &mut MatchedTile)>,
) {
    for (entity, mut matched) in matched_query.iter_mut() {
        matched.elapsed += time.delta_secs();

        if matched.elapsed >= matched.duration {
            // Animation complete - despawn the tile
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Handles cascade logic (tiles fall down to fill gaps)
pub fn cascade_tiles(
    mut commands: Commands,
    tile_query: Query<(Entity, &GridTile)>,
    config: Res<Stage2Config>,
) {
    // Build grid to find empty spaces
    let grid_size = config.grid_size;
    let mut grid: Vec<Vec<Option<Entity>>> = vec![vec![None; grid_size]; grid_size];

    for (entity, tile) in tile_query.iter() {
        let (row, col) = tile.grid_pos;
        grid[row][col] = Some(entity);
    }

    // For each column, move tiles down to fill gaps
    for col in 0..grid_size {
        let mut write_row = 0;

        for row in 0..grid_size {
            if let Some(entity) = grid[row][col] {
                if write_row != row {
                    // Tile needs to move down
                    if let Ok((_, mut tile)) = tile_query.get(entity).map(|(e, t)| (e, t)) {
                        commands.entity(entity).insert(CascadingTile {
                            target_pos: (write_row, col),
                            speed: 500.0, // pixels per second
                        });
                    }
                }
                write_row += 1;
            }
        }
    }
}

/// Spawns new tiles at the top to fill empty spaces
pub fn spawn_new_tiles(
    mut commands: Commands,
    tile_query: Query<&GridTile>,
    config: Res<Stage2Config>,
    asset_server: Res<AssetServer>,
) {
    let grid_size = config.grid_size;

    // Build occupancy grid
    let mut occupied = vec![vec![false; grid_size]; grid_size];

    for tile in tile_query.iter() {
        let (row, col) = tile.grid_pos;
        occupied[row][col] = true;
    }

    // Spawn new tiles for empty spaces at top
    for row in 0..grid_size {
        for col in 0..grid_size {
            if !occupied[row][col] {
                let letter = get_weighted_random_letter();
                let grid_pos = (row, col);

                // Calculate world position
                let total_width = (grid_size as f32) * (TILE_SIZE + GRID_SPACING);
                let start_x = -total_width / 2.0 + TILE_SIZE / 2.0;
                let start_y = -total_width / 2.0 + TILE_SIZE / 2.0;
                let x = start_x + (col as f32) * (TILE_SIZE + GRID_SPACING);
                let y = start_y + (row as f32) * (TILE_SIZE + GRID_SPACING);

                // Spawn tile with letter text as child
                commands.spawn((
                    GridTile {
                        letter,
                        grid_pos,
                        is_selected: false,
                        is_matched: false,
                    },
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.85, 0.85, 0.95),
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, y + 200.0, 0.0), // Spawn above
                        ..default()
                    },
                    CascadingTile {
                        target_pos: grid_pos,
                        speed: 500.0,
                    },
                    SpawnAnimation {
                        elapsed: 0.0,
                        duration: 0.3,
                    },
                )).with_children(|parent| {
                    // Spawn letter text as child so it inherits transformations
                    parent.spawn((
                        Text2d::new(letter.to_string()),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 42.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.1, 0.1, 0.1)),
                        Transform::from_xyz(0.0, 0.0, 1.0), // Relative to parent
                    ));
                });

                info!("Spawned new tile at ({},{}) with letter {}", row, col, letter);
            }
        }
    }
}

/// Checks if game is over (time expired or target reached)
pub fn check_game_over(
    state: Res<Stage2State>,
    config: Res<Stage2Config>,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_stage: ResMut<crate::plugins::state::LastStageCompleted>,
) {
    if !state.is_active {
        return;
    }

    // Check time expired
    if state.time_remaining_ms == 0 {
        if state.score >= config.target_score {
            info!("🎉 Level complete! Score: {} / {}", state.score, config.target_score);
        } else {
            info!("❌ Time's up! Score: {} / {}", state.score, config.target_score);
        }
        *last_stage = crate::plugins::state::LastStageCompleted::Stage2;
        next_state.set(GameState::Results);
    }

    // Check target score reached
    if state.score >= config.target_score {
        info!("🎯 Target score reached! Final score: {}", state.score);
        *last_stage = crate::plugins::state::LastStageCompleted::Stage2;
        next_state.set(GameState::Results);
    }

    // Check move limit (if applicable)
    if config.moves_limit > 0 && state.moves_made >= config.moves_limit {
        if state.score >= config.target_score {
            info!("🎉 Level complete!");
        } else {
            info!("❌ Out of moves!");
        }
        *last_stage = crate::plugins::state::LastStageCompleted::Stage2;
        next_state.set(GameState::Results);
    }
}

/// Updates score display
pub fn update_score_display(
    mut query: Query<&mut Text, With<ScoreDisplay>>,
    state: Res<Stage2State>,
    config: Res<Stage2Config>,
) {
    for mut text in query.iter_mut() {
        **text = format!("Score: {} / {}", state.score, config.target_score);
    }
}

/// Updates timer display
pub fn update_timer(
    mut query: Query<&mut Text, With<TimerDisplay>>,
    mut state: ResMut<Stage2State>,
    time: Res<Time>,
) {
    if !state.is_active {
        return;
    }

    let delta_ms = (time.delta_secs() * 1000.0) as u32;
    state.time_remaining_ms = state.time_remaining_ms.saturating_sub(delta_ms);

    for mut text in query.iter_mut() {
        let seconds = state.time_remaining_ms / 1000;
        **text = format!("Time: {}s", seconds);
    }
}

/// Updates moves display
pub fn update_moves_display(
    mut query: Query<&mut Text, With<MovesDisplay>>,
    state: Res<Stage2State>,
    config: Res<Stage2Config>,
) {
    for mut text in query.iter_mut() {
        if config.moves_limit > 0 {
            **text = format!("Moves: {} / {}",
                state.moves_made, config.moves_limit);
        } else {
            **text = format!("Moves: {}", state.moves_made);
        }
    }
}

/// Cleanup system to despawn all grid entities when leaving Stage2Playing
pub fn cleanup_stage2_gameplay(
    mut commands: Commands,
    grid_query: Query<Entity, With<GameGrid>>,
    tile_query: Query<Entity, With<GridTile>>,
    hud_query: Query<Entity, With<crate::stage2::ui::Stage2HUD>>,
) {
    // Despawn grid container
    for entity in grid_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn all tiles
    for entity in tile_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn HUD
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Returns a weighted random letter (standard tile distribution)
fn get_weighted_random_letter() -> char {
    let mut rng = rand::thread_rng();
    let roll: u32 = rng.gen_range(0..100);

    match roll {
        0..=11 => 'E',
        12..=20 => 'A',
        21..=28 => 'I',
        29..=36 => 'O',
        37..=43 => 'N',
        44..=49 => 'R',
        50..=55 => 'T',
        56..=59 => 'L',
        60..=63 => 'S',
        64..=67 => 'U',
        68..=70 => 'D',
        71..=73 => 'G',
        74..=76 => 'B',
        77..=79 => 'C',
        80..=82 => 'M',
        83..=85 => 'P',
        86..=88 => 'F',
        89..=90 => 'H',
        91..=92 => 'V',
        93..=94 => 'W',
        95..=96 => 'Y',
        97 => 'K',
        98 => 'J',
        99 => 'X',
        _ => 'Q',
    }
}
