use crate::object::Object;
use dyn_clone::{clone_trait_object, DynClone};
use rand::Rng;
use sdl2::pixels::Color;
pub trait Element<'a>: DynClone {
    fn name(&self) -> &'a str;
    fn has_gravity(&self) -> bool;
    fn simulate(&self, object: &mut Object<'a>);
    fn init(&self, object: &mut Object<'a>);
}
clone_trait_object!(Element<'_>);

#[derive(Clone)]
pub struct Sand;
#[derive(Clone)]

pub struct Wall;
#[derive(Clone)]

pub struct Fire;
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
    fn simulate(&self, object: &mut Object<'a>) {
        object.velocity.0 = rand::thread_rng().gen_range(-2..2);
    }
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
    fn simulate(&self, _object: &mut Object<'a>) {}
}
impl<'a> Element<'a> for Fire {
    fn name(&self) -> &'a str {
        "Fire"
    }

    fn has_gravity(&self) -> bool {
        false
    }
    fn init(&self, object: &mut Object<'a>) {
        object.color = Color::RGB(255, rand::thread_rng().gen_range(0..255), 0);
    }
    fn simulate(&self, object: &mut Object<'a>) {
        object.velocity.1 = -2;
        object.velocity.0 = rand::thread_rng().gen_range(-2..2);
        if rand::thread_rng().gen_range(1..10) == 1 {
            object.die = true;
        }
    }
}
// This is somewhat of a hack, but it's required until the never type gets stabilized
impl<'a> Element<'a> for () {
    fn name(&self) -> &'a str {
        unreachable!();
    }

    fn has_gravity(&self) -> bool {
        unreachable!();
    }

    fn simulate(&self, _object: &mut Object<'a>) {
        unreachable!();
    }

    fn init(&self, _object: &mut Object<'a>) {
        unreachable!();
    }
}
