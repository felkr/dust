use crate::element::{Element, Sand};
use crate::Object;
use num::PrimInt;
use sdl2::pixels::Color;
use std::convert::{TryFrom, TryInto};
use std::ops::Add;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ParticleStorage<'a>(pub Vec<Vec<Option<Object<'a>>>>);

impl ParticleStorage<'static> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self(vec![vec![None; rows]; columns])
        
    }
    pub fn place<T: PrimInt>(
        &mut self,
        x: T,
        y: T,
        element: &Option<Rc<(dyn Element<'static>)>>,
    ) -> Option<Object<'static>> {
        if let Some(element) = element {
            let mut o = Object::new(element.clone());
            o.clone().element.init(&mut o);
            self.0[x.to_usize().unwrap()][y.to_usize().unwrap()] = Some(o);
            return self.0[x.to_usize().unwrap()][y.to_usize().unwrap()].clone();
        } else {
            self.0[x.to_usize().unwrap()][y.to_usize().unwrap()] = None;
            return None;
        }
    }
    pub fn place_square<'a, T: PrimInt, E: Element<'a>>(
        &mut self,
        x: T,
        y: T,
        radius: T,
        element: Option<Rc<(dyn Element<'static>)>>,
    ) -> Vec<Option<Object<'static>>> {
        let mut placed = vec![];
        for i in 0..radius.to_usize().unwrap() {
            for j in 0..radius.to_usize().unwrap() {
                {
                    placed.push(self.place(
                        x.to_usize().unwrap() + i,
                        y.to_usize().unwrap() + j,
                        &element,
                    ));                    
                }
            }
        }
        return placed;
    }
}
