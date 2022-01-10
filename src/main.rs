use element::{Sand, Wall};
use object::Object;
use rand::Rng;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor, MouseButton, MouseWheelDirection, SystemCursor};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;
use sdl2::{event::Event, rect};
use std::collections::HashSet;
use std::convert::TryInto;
use std::rc::Rc;
use world::ParticleStorage;

mod element;
mod object;
mod world;

fn move_to(world: &mut ParticleStorage, x: usize, y: usize, dx: i32, dy: i32) -> (usize, usize) {
    if x as i32 + dx > WORLD_SIZE_COL as i32
        || y as i32 + dy > WORLD_SIZE_ROW as i32
        || x as i32 + dx < 0
        || y as i32 + dy < 0
    {
        world.0[x][y] = None;
        return (x, y);
    }
    if let None = world.0[(x as i32 + dx) as usize][(y as i32 + dy) as usize] {
        let temp = world.0[x][y].clone();
        world.0[x][y] = world.0[(x as i32 + dx) as usize][(y as i32 + dy) as usize].clone();
        world.0[(x as i32 + dx) as usize][(y as i32 + dy) as usize] = temp;
        return ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
    }
    return (0, 0);
}

fn simulate(world: &mut ParticleStorage) {
    let mut moved_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..world.0.len() - 1 {
        for j in 0..world.0[i].len() - 2 {
            if let Some(object) = &mut world.0[i][j] {
                object.clone().element.simulate(object);
                if !moved_positions.contains(&(i, j)) {
                    // && object.element.has_gravity() {
                    let (mut dx, mut dy) = object.velocity;
                    if object.element.has_gravity() {
                        dx += rand::thread_rng().gen_range(-2..2);
                        dy += 2;
                    }
                    moved_positions.insert(move_to(world, i, j, dx, dy));
                }
            }
        }
    }
}

fn render_particles(canvas: &mut WindowCanvas, world: &ParticleStorage) {
    for (x, row) in world.0.iter().enumerate() {
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

    let mut world = ParticleStorage::new(WORLD_SIZE_ROW, WORLD_SIZE_COL);
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
    let mut fps = 0;
    let mut brush_size = 10;
    let texture_creator = canvas.texture_creator();
    let surface = Surface::from_file("assets/cursor.png")
        .map_err(|err| format!("failed to load cursor image: {}", err))?;
    let cursor = Cursor::from_surface(surface, 0, 0)
        .map_err(|err| format!("failed to load cursor: {}", err))?;
    cursor.set();

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::MouseWheel { y, .. } => {
                    if brush_size > 2 {
                        brush_size += y;
                    }
                }
                _ => {}
            }
        }
        frame_time = sdl_context.timer().unwrap().ticks();

        let state = event_pump.mouse_state();
        match event_pump
            .mouse_state()
            .pressed_mouse_buttons()
            .collect::<Vec<_>>()
            .first()
        {
            Some(MouseButton::Left) => drop(world.place_square::<_, Sand>(
                state.x(),
                state.y(),
                brush_size,
                Some(Rc::new(Sand)),
            )),
            Some(MouseButton::Right) => drop(world.place_square::<_, Wall>(
                state.x(),
                state.y(),
                brush_size,
                Some(Rc::new(Wall)),
            )),
            Some(MouseButton::Middle) => {
                drop(world.place_square::<_, ()>(state.x(), state.y(), brush_size, None))
            }
            _ => {}
        }

        simulate(&mut world);
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        render_particles(&mut canvas, &world);

        // let brush_size = 100;
        // sdl_context.mouse().show_cursor(false);
        // canvas.draw_rect(Rect::new(event_pump.mouse_state().x()-brush_size/2, event_pump.mouse_state().y()-brush_size/2, brush_size.try_into().unwrap(), brush_size.try_into().unwrap())).unwrap();
        // canvas.copy(&texture, None, Some(Rect::new(event_pump.mouse_state().x()-brush_size/2, event_pump.mouse_state().y()-brush_size/2, brush_size.try_into().unwrap(), brush_size.try_into().unwrap())))?;

        canvas.present();

        if sdl_context.timer().unwrap().ticks() - frame_time > 0 {
            fps = 1000 / (sdl_context.timer().unwrap().ticks() - frame_time);
        }
        canvas
            .window_mut()
            .set_title(format!("dust - {} fps", fps).as_str())
            .unwrap();
        // Time management!
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
