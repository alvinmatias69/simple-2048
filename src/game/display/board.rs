use super::config;
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use graphics::{Mesh, Text, TextFragment};

pub fn draw_board(ctx: &mut Context) -> GameResult {
    let (x, y) = config::BOARD_POSITION;

    let rect = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(x, y, config::BOARD_SIZE, config::BOARD_SIZE),
        graphics::Color::from_rgb(187, 173, 160),
    )?;
    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
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

    let rect = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(x, y, config::TILE_SIZE, config::TILE_SIZE),
        graphics::Color::from_rgb(238, 228, 218),
    )?;
    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

    if value > 0 {
        let text_fragment = TextFragment::new(value.to_string())
            .color(graphics::Color::from_rgb(110, 101, 91))
            .scale(graphics::Scale::uniform(config::TILE_SIZE / 2.));
        let text = Text::new(text_fragment);

        graphics::draw(
            ctx,
            &text,
            graphics::DrawParam::default().dest(Point2::new(x, y)),
        )?;
    }
    Ok(())
}
