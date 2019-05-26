use super::{config, helper};
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use graphics::{Mesh, Text, TextFragment};

pub fn draw_board(ctx: &mut Context) -> GameResult {
    helper::draw_rounded_rectangle(
        ctx,
        config::BOARD_POSITION,
        config::BOARD_SIZE,
        config::BOARD_SIZE,
        config::TILE_RADIUS,
        graphics::Color::from_rgb(187, 173, 160),
    )?;

    Ok(())
}

pub fn draw_tiles(ctx: &mut Context, field: &Vec<Vec<u32>>) -> GameResult {
    for (y_idx, y) in field.iter().enumerate() {
        let y_pos = config::BOARD_POSITION.1
            + config::TILE_MARGIN
            + y_idx as f32 * (config::TILE_MARGIN + config::TILE_SIZE);
        for (x_idx, x) in y.iter().enumerate() {
            let x_pos = config::BOARD_POSITION.0
                + config::TILE_MARGIN
                + x_idx as f32 * (config::TILE_MARGIN + config::TILE_SIZE);
            draw_tile(ctx, (x_pos, y_pos), *x)?;
        }
    }

    Ok(())
}

fn draw_tile(ctx: &mut Context, position: (f32, f32), value: u32) -> GameResult {
    let (x, y) = position;

    helper::draw_rounded_rectangle(
        ctx,
        position,
        config::TILE_SIZE,
        config::TILE_SIZE,
        config::TILE_RADIUS,
        tile_color(value),
    )?;

    if value > 0 {
        let text_fragment = TextFragment::new(value.to_string())
            .color(graphics::Color::from_rgb(173, 143, 135))
            .scale(graphics::Scale::uniform(config::TILE_SIZE / 2.));
        let mut text = Text::new(text_fragment);
        text.set_bounds(
            Point2::new(config::TILE_SIZE, config::TILE_SIZE),
            graphics::Align::Center,
        );

        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::default().dest(Point2::new(x, y + config::TILE_SIZE / 4.)),
        )?;
    }
    Ok(())
}

fn tile_color(value: u32) -> graphics::Color {
    let rgb: Vec<u8>;

    match value {
        2 => rgb = vec![238, 236, 187],
        4 => rgb = vec![241, 229, 179],
        8 => rgb = vec![244, 210, 172],
        16 => rgb = vec![242, 192, 165],
        32 => rgb = vec![236, 183, 167],
        64 => rgb = vec![238, 183, 180],
        128 => rgb = vec![237, 183, 196],
        256 => rgb = vec![236, 184, 207],
        512 => rgb = vec![217, 178, 207],
        1024 => rgb = vec![197, 173, 207],
        2048 => rgb = vec![175, 167, 206],
        _ => rgb = vec![205, 193, 181],
    }

    graphics::Color::from_rgb(rgb[0], rgb[1], rgb[2])
}
