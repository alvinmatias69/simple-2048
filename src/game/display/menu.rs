use super::{config, helper};

use ggez::nalgebra::Point2;
use ggez::{graphics, input, Context, GameResult};
use graphics::{Mesh, Text, TextFragment};
use input::mouse;

pub fn draw(ctx: &mut Context, win: bool) -> GameResult {
    let color: graphics::Color;
    if win {
        color = graphics::Color::from_rgba(255, 204, 0, 100);
    } else {
        color = graphics::Color::from_rgba(128, 128, 128, 100);
    }

    let rect = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0., 0., config::WINDOW_WIDTH, config::WINDOW_HEIGHT),
        color,
    )?;
    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

    draw_text(ctx, win)?;
    draw_button(ctx, win)?;
    setup_mouse(ctx)?;

    Ok(())
}

fn draw_text(ctx: &mut Context, win: bool) -> GameResult {
    let content: String;
    if win {
        content = String::from("YOU WIN");
    } else {
        content = String::from("GAME OVER");
    }

    let text_fragment = TextFragment::new(content)
        .color(graphics::Color::from_rgb(255, 255, 255))
        .scale(graphics::Scale::uniform(config::TITLE_SIZE));
    let mut text = Text::new(text_fragment);
    text.set_bounds(
        Point2::new(config::WINDOW_WIDTH, config::WINDOW_HEIGHT),
        graphics::Align::Center,
    );

    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default().dest(Point2::new(0.0, config::WINDOW_HEIGHT / 3.)),
    )?;

    Ok(())
}

fn draw_button(ctx: &mut Context, win: bool) -> GameResult {
    helper::draw_rounded_rectangle(
        ctx,
        config::MENU_BUTTON_POSITION,
        config::MENU_BUTTON_SIZE.0,
        config::MENU_BUTTON_SIZE.1,
        config::TILE_RADIUS,
        graphics::Color::from_rgb(143, 122, 102),
    )?;

    draw_button_text(ctx, win)?;
    Ok(())
}

fn draw_button_text(ctx: &mut Context, win: bool) -> GameResult {
    let content: String;
    if win {
        content = String::from("Keep going");
    } else {
        content = String::from("Try again");
    }

    let text_fragment = TextFragment::new(content)
        .color(graphics::Color::from_rgb(255, 255, 255))
        .scale(graphics::Scale::uniform(config::MENU_BUTTON_TEXT));
    let mut text = Text::new(text_fragment);
    text.set_bounds(
        Point2::new(config::MENU_BUTTON_SIZE.0, config::MENU_BUTTON_SIZE.1),
        graphics::Align::Center,
    );

    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default().dest(Point2::new(
            config::MENU_BUTTON_POSITION.0,
            config::MENU_BUTTON_POSITION.1 + config::MARGIN * 0.5,
        )),
    )?;

    Ok(())
}

fn setup_mouse(ctx: &mut Context) -> GameResult {
    mouse::set_position(
        ctx,
        Point2::new(
            config::MENU_BUTTON_POSITION.0 + config::MENU_BUTTON_SIZE.0 / 2.,
            config::MENU_BUTTON_POSITION.1 + config::MENU_BUTTON_SIZE.1 / 2.,
        ),
    )?;
    mouse::set_cursor_type(ctx, mouse::MouseCursor::Hand);
    mouse::set_cursor_hidden(ctx, false);

    Ok(())
}
