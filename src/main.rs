use element::{Element, Fire, Sand, Wall};
use object::Object;
use rand::Rng;
use rayon::prelude::*;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor, MouseButton, MouseWheelDirection, SystemCursor};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::surface::{self, Surface};
use sdl2::{event::Event, rect};
use std::collections::HashSet;
use std::convert::TryInto;
use std::rc::Rc;
use world::World;

use crate::gui::render_label;

mod element;
mod gui;
mod object;
mod world;

fn move_to(world: &mut World, mut x: usize, mut y: usize, dx: i32, dy: i32) -> (usize, usize) {
    if x as i32 + dx < 0 || y as i32 + dy < 0 {
        world.particles[x][y] = None;
        return (x, y);
    }
    if x as i32 + dx >= WORLD_SIZE_COL as i32 {
        x -= (x + dx as usize - WORLD_SIZE_COL) as usize + 1;
    }
    if y as i32 + dy >= WORLD_SIZE_ROW as i32 {
        y -= (y + dy as usize - WORLD_SIZE_ROW) as usize + 1;
    }
    if let None = world.particles[(x as i32 + dx) as usize][(y as i32 + dy) as usize] {
        let temp = world.particles[x][y].clone();
        world.particles[x][y] =
            world.particles[(x as i32 + dx) as usize][(y as i32 + dy) as usize].clone();
        world.particles[(x as i32 + dx) as usize][(y as i32 + dy) as usize] = temp;
        return ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
    }
    return (0, 0);
}

fn simulate(world: &mut World) {
    let mut moved_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..world.particles.len() - 1 {
        for j in 0..world.particles[i].len() - 2 {
            if let Some(object) = &mut world.particles[i][j] {
                object.clone().element.simulate(object);
                if !moved_positions.contains(&(i, j)) {
                    // && object.element.has_gravity() {
                    let (mut dx, mut dy) = object.velocity;
                    if object.element.has_gravity() {
                        dy = (0.25 * world.delta_time) as i32;
                    }
                    if object.die {
                        world.particles[i][j] = None;
                    }
                    moved_positions.insert(move_to(world, i, j, dx, dy));
                }
            }
        }
    }
}

fn render_particles(canvas: &mut WindowCanvas, world: &World) {
    for (x, row) in world.particles.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if let Some(object) = col {
                canvas
                    .pixel(x.try_into().unwrap(), y.try_into().unwrap(), object.color)
                    .unwrap();
            }
        }
    }
}

const WORLD_SIZE_ROW: usize = 600;
const WORLD_SIZE_COL: usize = 800;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut world = World::new(WORLD_SIZE_ROW, WORLD_SIZE_COL);
    let window = video_subsystem
        .window("dust", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_time;
    let mut last_pc;
    let mut fps = 0;
    let mut brush_size = 10;
    let mut paused = false;
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let surface = Surface::from_file("assets/cursor.png")
        .map_err(|err| format!("failed to load cursor image: {}", err))?;
    let cursor = Cursor::from_surface(surface, 0, 0)
        .map_err(|err| format!("failed to load cursor: {}", err))?;
    cursor.set();
    let elements: Vec<Rc<dyn Element>> = vec![Rc::new(Wall), Rc::new(Sand), Rc::new(Fire)];
    let mut current_element: usize = 0;
    let mut font = ttf_context.load_font("assets/trim.ttf", 36)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    let mut hud_text_surface = font
        .render(elements[current_element].name())
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())?;
    let element_cursor = Surface::from_file("assets/element_cursor.png").unwrap();
    let element_cursor_texture = texture_creator
        .create_texture_from_surface(&element_cursor)
        .unwrap();

    macro_rules! render_hud_text {
        () => {
            hud_text_surface = font
                .render(format!("{}", elements[current_element].name()).as_str())
                .blended(Color::RGBA(255, 255, 255, 255))
                .map_err(|e| e.to_string())?;
        };
    }
    render_hud_text!();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    if current_element < elements.len() - 1 {
                        current_element += 1;
                    } else {
                        current_element = 0;
                    }
                    render_hud_text!();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    for (x, row) in world.particles.iter_mut().enumerate() {
                        for (y, col) in row.iter_mut().enumerate() {
                            *col = None;
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    paused = !paused;
                }
                Event::MouseWheel { y, .. } => {
                    if brush_size > 2 {
                        brush_size += y;
                    } else {
                        brush_size = 3;
                    }
                    render_hud_text!();
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    world.last_dot = (x, y);
                }
                _ => {}
            }
        }
        frame_time = sdl_context.timer().unwrap().ticks();
        last_pc = sdl_context.timer().unwrap().performance_counter();

        let state = event_pump.mouse_state();
        match event_pump
            .mouse_state()
            .pressed_mouse_buttons()
            .collect::<Vec<_>>()
            .first()
        {
            Some(MouseButton::Left) => drop(world.draw::<_, Sand>(
                state.x(),
                state.y(),
                brush_size,
                Some(elements[current_element].clone()),
            )),
            Some(MouseButton::Right) => {
                drop(world.draw::<_, ()>(state.x(), state.y(), brush_size, None))
            }
            _ => {}
        }
        if !paused {
            simulate(&mut world);
        }
        canvas.set_draw_color(Color::BLACK);
        sdl_context.mouse().show_cursor(false);
        canvas.clear();
        let hud_text_texture = texture_creator
            .create_texture_from_surface(&hud_text_surface)
            .map_err(|e| e.to_string())?;
        canvas.copy(
            &hud_text_texture,
            None,
            Rect::new(0, 0, hud_text_surface.width(), hud_text_surface.height()),
        )?;
        canvas.copy(
            &element_cursor_texture,
            None,
            Rect::new(
                state.x(),
                state.y(),
                brush_size.try_into().unwrap(),
                brush_size.try_into().unwrap(),
            ),
        )?;
        render_particles(&mut canvas, &world);

        canvas.present();

        if sdl_context.timer().unwrap().ticks() - frame_time > 0 {
            fps = 1000 / (sdl_context.timer().unwrap().ticks() - frame_time);
            world.delta_time = (sdl_context.timer().unwrap().ticks() - frame_time) as f32;
        }
        canvas
            .window_mut()
            .set_title(format!("dust - {} fps | {} delta", fps, world.delta_time).as_str())
            .unwrap();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
