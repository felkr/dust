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
use std::mem;
use std::ptr::NonNull;
use std::time::Duration;
macro_rules! swap_value {
    ($a_ref:expr, $b_ref:expr) => {{
        let t = *$a_ref;
        *$a_ref = *$b_ref;
        *$b_ref = t;
    }};
}
fn move_to(world: &mut World, x: usize, y: usize, dx: i32, dy: i32) -> (usize, usize) {
    // return the moved position
    if let None = world[(x as i32 + dx) as usize][(y as i32 + dy) as usize] {
        swap_value!(
            &mut world[x][y],
            &mut world[(x as i32 + dx) as usize][(y as i32 + dy) as usize]
        );
        return ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
    }
    return (0, 0);
}
fn simulate(world: &mut World) {
    let mut moved_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..world.len() - 1 {
        for j in 0..world[i].len() - 2 {
            if let Some(object) = world[i][j] {
                // if let None = world[i][j + 1] {
                //     if !moved_positions.contains(&(i, j)) && object.has_gravity {
                //         // println!("Swapping {},{} with {},{}", i, j, i, j + 1);
                //         swap_value!(&mut world[i][j], &mut world[i][j + 1]);
                //         moved_positions.push((i, j + 1));
                //     }
                // }
                if !moved_positions.contains(&(i, j)) && object.has_gravity {
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
fn render(canvas: &mut WindowCanvas, world: &World) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for (x, row) in world.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if let Some(object) = col {
                canvas.set_draw_color(object.appearance.color);
                canvas
                    .fill_rect(Rect::new(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        object.appearance.size_x.into(),
                        object.appearance.size_y.into(),
                    ))
                    .unwrap();
            }
        }
    }

    canvas.present();
}
#[derive(Debug, Copy, Clone)]
struct Appearance {
    color: Color,
    size_x: u16,
    size_y: u16,
}
#[derive(Debug, Copy, Clone)]
struct Object {
    appearance: Appearance,
    has_gravity: bool,
}

impl Object {
    fn new() -> Object {
        Object {
            appearance: Appearance {
                color: Color::WHITE,
                size_x: 1,
                size_y: 1,
            },
            has_gravity: true,
        }
    }
}
const WORLD_SIZE_ROW: usize = 600;
const WORLD_SIZE_COL: usize = 800;
type World = Box<[[Option<Object>; WORLD_SIZE_ROW]; WORLD_SIZE_COL]>;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut world: World = Box::new([[None; WORLD_SIZE_ROW]; WORLD_SIZE_COL]);

    let mut window = video_subsystem
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
            let mut state = event_pump.mouse_state();
            for i in 0..10 {
                for j in 0..10 {
                    world[state.x() as usize + i][state.y() as usize + j] = Some(Object {
                        appearance: Appearance {
                            color: Color::RGB(201, 193, 181),
                            size_x: 1,
                            size_y: 1,
                        },
                        has_gravity: true,
                    });
                }
            }
        }
        if event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Right)
        {
            let mut state = event_pump.mouse_state();

            for i in 0..10 {
                for j in 0..10 {
                    world[state.x() as usize + i][state.y() as usize + j] = Some(Object {
                        appearance: Appearance {
                            color: Color::GRAY,
                            size_x: 1,
                            size_y: 1,
                        },
                        has_gravity: false,
                    });
                }
            }
        }
        if event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Middle)
        {
            let mut state = event_pump.mouse_state();

            for i in 0..10 {
                for j in 0..10 {
                    world[state.x() as usize + i][state.y() as usize + j] = None;
                }
            }
        }

        simulate(&mut world);
        render(&mut canvas, &world);
        if sdl_context.timer().unwrap().ticks() - frame_time > 0 {
            fps = 1000 / (sdl_context.timer().unwrap().ticks() - frame_time);
        }
        canvas
            .window_mut()
            .set_title(format!("dust - {} fps", fps).as_str());
        // Time management!
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
