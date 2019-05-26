use ggez::{graphics, nalgebra, Context, GameResult};
use graphics::Color;
use nalgebra::Point2;

pub fn draw_rounded_rectangle(
    ctx: &mut Context,
    position: (f32, f32),
    width: f32,
    height: f32,
    radius: f32,
    color: Color,
) -> GameResult {
    let mode = graphics::DrawMode::fill();
    let rounded_rectangle = graphics::MeshBuilder::new()
        .rectangle(
            mode,
            graphics::Rect::new(radius, 0., width - 2. * radius, height),
            color,
        )
        .rectangle(
            mode,
            graphics::Rect::new(0., radius, width, height - 2. * radius),
            color,
        )
        .circle(mode, Point2::new(radius, radius), radius, 0.01, color)
        .circle(
            mode,
            Point2::new(width - radius, radius),
            radius,
            0.01,
            color,
        )
        .circle(
            mode,
            Point2::new(radius, height - radius),
            radius,
            0.01,
            color,
        )
        .circle(
            mode,
            Point2::new(width - radius, height - radius),
            radius,
            0.01,
            color,
        )
        .build(ctx)?;

    graphics::draw(
        ctx,
        &rounded_rectangle,
        graphics::DrawParam::default().dest(Point2::new(position.0, position.1)),
    )?;

    Ok(())
}
