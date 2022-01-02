use dyn_clone::{clone_trait_object, DynClone};
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
use std::{ptr::NonNull, time::Duration};
#[macro_use]
extern crate lazy_static;

macro_rules! swap_value {
    ($a_ref:expr, $b_ref:expr) => {{
        let t = *$a_ref;
        *$a_ref = *$b_ref;
        *$b_ref = t;
    }};
}
fn move_to(world: &mut World, x: usize, y: usize, dx: i32, dy: i32) -> (usize, usize) {
    if x as i32 + dx > WORLD_SIZE_COL as i32
        || y as i32 + dy > WORLD_SIZE_ROW as i32
        || x as i32 + dx < 0
        || y as i32 + dy < 0
    {
        world[x][y] = None;
        return (x, y);
    }
    if let None = world[(x as i32 + dx) as usize][(y as i32 + dy) as usize] {
        // swap_value!(
        //     &mut world[x][y],
        //     &mut world[(x as i32 + dx) as usize][(y as i32 + dy) as usize]
        // );
        let temp = world[x][y].clone();
        world[x][y] = world[(x as i32 + dx) as usize][(y as i32 + dy) as usize].clone();
        world[(x as i32 + dx) as usize][(y as i32 + dy) as usize] = temp;
        return ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
    }
    return (0, 0);
}
fn simulate(world: &mut World) {
    let mut moved_positions: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..world.len() - 1 {
        for j in 0..world[i].len() - 2 {
            if let Some(object) = &mut world[i][j] {
                // if let None = world[i][j + 1] {
                //     if !moved_positions.contains(&(i, j)) && object.has_gravity {
                //         // println!("Swapping {},{} with {},{}", i, j, i, j + 1);
                //         swap_value!(&mut world[i][j], &mut world[i][j + 1]);
                //         moved_positions.push((i, j + 1));
                //     }
                // }
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
fn render(canvas: &mut WindowCanvas, world: &World) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for (x, row) in world.iter().enumerate() {
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

trait Element<'a>: DynClone {
    fn name(&self) -> &'a str;
    fn has_gravity(&self) -> bool;
    fn simulate(&self, object: &mut Object<'a>);
    fn init(&self, object: &mut Object<'a>);
}
clone_trait_object!(Element<'_>);
#[derive(Clone)]
struct Object<'a> {
    velocity: (i32, i32),
    color: Color,
    element: Rc<dyn Element<'a>>,
}
impl Default for Object<'_> {
    fn default() -> Self {
        Object {
            velocity: (0, 0),
            color: Color::RGB(0, 0, 0),
            element: Rc::new(Sand),
        }
    }
}

#[derive(Clone)]
struct Sand;
#[derive(Clone)]

struct Wall;
impl<'a> Element<'a> for Sand {
    fn name(&self) -> &'a str {
        "Sand"
    }

    fn has_gravity(&self) -> bool {
        true
    }
    fn init(&self, object: &mut Object<'a>) {
        object.color = Color::RGB(201, 193, 181);
    }
    fn simulate(&self, object: &mut Object<'a>) {}
}
impl<'a> Element<'a> for Wall {
    fn name(&self) -> &'a str {
        "Wall"
    }

    fn has_gravity(&self) -> bool {
        false
    }
    fn init(&self, object: &mut Object<'a>) {
        object.color = Color::GREY;
    }
    fn simulate(&self, object: &mut Object<'a>) {}
}
impl<'a> Object<'a> {
    fn new(element: Rc<(dyn Element<'a> + 'static)>) -> Object<'a> {
        let mut obj = Object {
            element,
            velocity: (0, 0),
            color: Color::WHITE,
        };
        // obj.element.init(&mut obj);
        obj
    }
}

const WORLD_SIZE_ROW: usize = 600;
const WORLD_SIZE_COL: usize = 800;
// type World<'a> = Box<[[Option<Object<'a>>; WORLD_SIZE_ROW]; WORLD_SIZE_COL]>;
type World<'a> = Vec<Vec<Option<Object<'a>>>>;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // let mut world: World = Box::new([[INIT; WORLD_SIZE_ROW]; WORLD_SIZE_COL]);
    // let mut w2: Vec<Vec<Option<Object>>> = Vec::new();
    let mut world: World = vec![vec![None; WORLD_SIZE_ROW]; WORLD_SIZE_COL];

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
                    let mut o = Object::new(Rc::new(Sand));
                    o.clone().element.init(&mut o);
                    world[state.x() as usize + i][state.y() as usize + j] = Some(o);
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
                    let mut o = Object::new(Rc::new(Wall));
                    o.clone().element.init(&mut o);
                    world[state.x() as usize + i][state.y() as usize + j] = Some(o);
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
