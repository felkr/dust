use std::rc::Rc;

use crate::element::{Element, Sand};
use sdl2::pixels::Color;
#[derive(Clone)]
pub struct Object<'a> {
    pub velocity: (i32, i32),
    pub color: Color,
    pub element: Rc<dyn Element<'a>>,
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
impl<'a> Object<'a> {
    pub fn new(element: Rc<(dyn Element<'a>)>) -> Self {
        let mut obj = Object {
            element,
            velocity: (0, 0),
            color: Color::WHITE,
        };
        obj.clone().element.init(&mut obj);
        obj
    }
}
