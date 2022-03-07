use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, ttf::Font};

pub fn render_label(
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    font: &Font,
    color: Color,
    text: &str,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    let text_surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;

    let text_texture = texture_creator
        .create_texture_from_surface(&text_surface)
        .map_err(|e| e.to_string())?;
    let (width, height) = font.size_of(text).map_err(|e| e.to_string())?;
    canvas.copy(&text_texture, None, Rect::new(x, y, width, height))?;
    Ok(())
}
