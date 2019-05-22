use super::config;
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};
use graphics::{draw, Color, DrawParam, Mesh, Scale, Text, TextFragment};

pub fn draw_title(ctx: &mut Context) -> GameResult {
    let text_fragment = TextFragment::new("2048")
        .color(Color::from_rgb(97, 101, 102))
        .scale(Scale::uniform(config::TITLE_SIZE));
    let text = Text::new(text_fragment);

    let (x, y) = config::TITLE_POSITION;
    draw(ctx, &text, DrawParam::default().dest(Point2::new(x, y)))?;

    Ok(())
}

pub fn draw_subtitle(ctx: &mut Context) -> GameResult {
    let text_fragment = TextFragment::new("Join the number and get to the 2048 tile!")
        .color(Color::from_rgb(97, 101, 102))
        .scale(Scale::uniform(config::SUBTITLE_SIZE));
    let text = Text::new(text_fragment);

    let (x, y) = config::SUBTITLE_POSITION;
    draw(ctx, &text, DrawParam::default().dest(Point2::new(x, y)))?;

    Ok(())
}

pub fn draw_score(ctx: &mut Context, score: u32) -> GameResult {
    let (x, y) = config::SCOREBOARD_POSITION;
    let rect = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(x, y, config::SCOREBOARD_WIDTH, config::SCOREBOARD_HEIGHT),
        Color::from_rgb(187, 173, 160),
    )?;
    draw(ctx, &rect, DrawParam::default())?;

    draw_score_title(ctx)?;
    draw_score_points(ctx, score)?;

    Ok(())
}

fn draw_score_title(ctx: &mut Context) -> GameResult {
    let text_fragment = TextFragment::new("Score")
        .color(Color::from_rgb(173, 166, 158))
        .scale(Scale::uniform(config::SCORE_TITLE_SIZE));
    let mut text = Text::new(text_fragment);
    text.set_bounds(
        Point2::new(config::SCORE_BOUNDING.0, config::SCORE_BOUNDING.1),
        graphics::Align::Center,
    );

    draw(
        ctx,
        &text,
        DrawParam::default().dest(Point2::new(
            config::SCORE_TITLE_POSITION.0,
            config::SCORE_TITLE_POSITION.1,
        )),
    )?;

    Ok(())
}

fn draw_score_points(ctx: &mut Context, score: u32) -> GameResult {
    let text_fragment = TextFragment::new(score.to_string())
        .color(graphics::WHITE)
        .scale(Scale::uniform(config::SCORE_SIZE));
    let mut text = Text::new(text_fragment);
    text.set_bounds(
        Point2::new(config::SCORE_BOUNDING.0, config::SCORE_BOUNDING.1),
        graphics::Align::Center,
    );

    draw(
        ctx,
        &text,
        DrawParam::default().dest(Point2::new(
            config::SCORE_POSITION.0,
            config::SCORE_POSITION.1,
        )),
    )?;

    Ok(())
}
