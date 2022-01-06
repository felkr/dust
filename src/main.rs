use dyn_clone::{clone_trait_object, DynClone};
use element::{Sand, Wall};
use object::Object;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryInto;
use std::f32::consts::E;
use std::mem;
use std::rc::Rc;
use std::sync::Arc;
use std::{ptr::NonNull, time::Duration};
use world::{ParticleStorage};

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
                if !moved_positions.contains(&(i, j)) && object.element.has_gravity() {
                    moved_positions.insert(move_to(
                        world,
                        i,
                        j,
                        rand::thread_rng().gen_range(-2..2),
                        2,
                    ));
                }
            }
        }
    }
}

fn render(canvas: &mut WindowCanvas, world: &ParticleStorage) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for (x, row) in world.0.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if let Some(object) = col {
                canvas.set_draw_color(object.color);
                canvas
                    .fill_rect(Rect::new(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        1,
                        1,
                    ))
                    .unwrap();
            }
        }
    }

    canvas.present();
}

const WORLD_SIZE_ROW: usize = 600;
const WORLD_SIZE_COL: usize = 800;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut world = ParticleStorage::new(WORLD_SIZE_ROW, WORLD_SIZE_ROW);
    let window = video_subsystem
        .window("dust", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_time = 0;
    let mut fps = 0;
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
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    println!("{}", fps);
                }
                _ => {}
            }
        }
        frame_time = sdl_context.timer().unwrap().ticks();

        if event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Left)
        {
            let state = event_pump.mouse_state();
            // world.place(1, 1, Some(Rc::new(Sand)));
            world.place_square::<_, Sand>(state.x(), state.y(), 5, Some(Rc::new(Sand)));
        }
        if event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Right)
        {
            let state = event_pump.mouse_state();

            world.place_square::<_, Sand>(state.x(), state.y(), 5, Some(Rc::new(Wall)));
        }
        if event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Middle)
        {
            let state = event_pump.mouse_state();

            world.place_square::<_, ()>(state.x(), state.y(), 5, None);
        }

        simulate(&mut world);
        render(&mut canvas, &world);
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
