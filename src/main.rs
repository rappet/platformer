#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cargo_common_metadata)]

mod entity;
mod tile;
mod world;

use crate::entity::{Entity, EntityId, EntityType};
use crate::world::{World, TEST_WORLD_STR, TILE_HEIGHT, TILE_WIDTH};
use macroquad::prelude::*;

fn create_camera(x: f32, y: f32) -> Camera2D {
    let mut factor = screen_width() / 240.;
    if factor < 3. {
        factor = factor.floor().max(1.);
    }

    Camera2D {
        rotation: 0.0,
        zoom: Vec2::new(1. / screen_width() * factor, -1. / screen_height() * factor),
        target: Vec2::new(x, y),
        offset: Vec2::new(0., 0.),
        render_target: None,
        viewport: None,
    }
}

fn update(world: &mut World) {
    let player = world
        .entities
        .get_mut(&EntityId(1))
        .expect("Player is always present");

    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        player.y -= 60. / 16. / 10.;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        player.y += 60. / 16. / 10.;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        player.x -= 60. / 16. / 10.;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        player.x += 60. / 16. / 10.;
    }
}

fn render(texture: Texture2D, world: &World) {
    let player = world
        .entities
        .get(&EntityId(1))
        .expect("Player is always present");

    clear_background(Color::from_rgba(57, 120, 168, 255));

    let camera = create_camera(player.x.mul_add(16., 8.), player.y.mul_add(16., 8.));
    set_camera(&camera);

    for (&(x, y), &tile) in &world.tiles {
        let texture_offset = tile.texture_offset();
        #[allow(clippy::cast_lossless, clippy::cast_precision_loss)]
        draw_texture_ex(
            texture,
            x as f32 * TILE_WIDTH as f32,
            y as f32 * TILE_HEIGHT as f32,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                source: Some(Rect {
                    x: texture_offset.0 as f32 * TILE_WIDTH as f32,
                    y: texture_offset.1 as f32 * TILE_HEIGHT as f32,
                    w: TILE_WIDTH as f32,
                    h: TILE_HEIGHT as f32,
                }),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    for entity in world.entities.values() {
        let texture_offset = entity.entity_type.texture_offset();
        #[allow(clippy::cast_lossless, clippy::cast_precision_loss)]
        draw_texture_ex(
            texture,
            entity.x * TILE_WIDTH as f32,
            entity.y * TILE_HEIGHT as f32,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                source: Some(Rect {
                    x: texture_offset.0 as f32 * TILE_WIDTH as f32,
                    y: texture_offset.1 as f32 * TILE_HEIGHT as f32,
                    w: TILE_WIDTH as f32,
                    h: TILE_HEIGHT as f32,
                }),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }
}

#[macroquad::main("platformer")]
async fn main() {
    let mut world = World::from_string_tilemap(TEST_WORLD_STR);
    world.spawn_entity(Entity {
        entity_type: EntityType::Player,
        x: 2.0,
        y: 0.0,
        dx: 0.0,
        dy: 0.0,
    });

    let texture = Texture2D::from_file_with_format(
        include_bytes!("../assets/sprites/tileset.png"),
        Some(ImageFormat::Png),
    );
    texture.set_filter(FilterMode::Nearest);

    loop {
        update(&mut world);
        render(texture, &world);

        next_frame().await;
    }
}
