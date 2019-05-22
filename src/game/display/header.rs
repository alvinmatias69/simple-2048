use super::config;
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use graphics::{draw, Color, DrawParam, Mesh, Scale, Text, TextFragment};

pub fn draw_title(ctx: &mut Context) -> GameResult {
    let text_fragment = TextFragment::new("2048")
        .color(Color::from_rgb(46, 43, 39))
        .scale(Scale::uniform(50.0));
    let text = Text::new(text_fragment);

    draw(
        ctx,
        &text,
        DrawParam::default().dest(Point2::new(config::MARGIN, config::MARGIN)),
    )?;

    Ok(())
}

pub fn draw_subtitle(ctx: &mut Context) -> GameResult {
    let text_fragment = TextFragment::new("Join the number and get to the 2048 tile!")
        .color(Color::from_rgb(46, 43, 39))
        .scale(Scale::uniform(15.0));
    let text = Text::new(text_fragment);

    draw(
        ctx,
        &text,
        DrawParam::default().dest(Point2::new(config::MARGIN, 70.0)),
    )?;

    Ok(())
}

pub fn draw_score(ctx: &mut Context, score: u32) -> GameResult {
    let screen = graphics::screen_coordinates(ctx);
    let box_x = screen.x + screen.w - 115.0;

    let rect = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(box_x, config::MARGIN, 95., 45.),
        Color::from_rgb(187, 173, 160),
    )?;
    draw(ctx, &rect, DrawParam::default())?;

    let text_fragment = TextFragment::new(score.to_string())
        .color(graphics::WHITE)
        .scale(Scale::uniform(20.));
    let text = Text::new(text_fragment);
    let score_x = box_x + (95. - text.width(ctx) as f32) / 2.;

    draw(
        ctx,
        &text,
        DrawParam::default().dest(Point2::new(score_x, 27.5)),
    )?;

    Ok(())
}
