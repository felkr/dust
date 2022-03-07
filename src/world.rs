use crate::element::Element;
use crate::Object;
use num::PrimInt;

use std::rc::Rc;

pub struct World<'a> {
    pub particles: Vec<Vec<Option<Object<'a>>>>,
    pub delta_time: f32,
    pub last_dot: (i32, i32),
}

impl World<'static> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            particles: vec![vec![None; rows]; columns],
            delta_time: 0f32,
            last_dot: (0, 0),
        }
    }
    pub fn place<T: PrimInt>(
        &mut self,
        x: T,
        y: T,
        element: &Option<Rc<(dyn Element<'static>)>>,
    ) -> Option<Object<'static>> {
        if x.to_usize().unwrap() >= self.particles.len()
            || y.to_usize().unwrap() >= self.particles[0].len()
        {
            return None;
        }
        if let Some(element) = element {
            if self.particles[x.to_usize().unwrap()][y.to_usize().unwrap()].is_none() {
                let mut o = Object::new(element.clone());
                o.clone().element.init(&mut o);
                self.particles[x.to_usize().unwrap()][y.to_usize().unwrap()] = Some(o);
                return self.particles[x.to_usize().unwrap()][y.to_usize().unwrap()].clone();
            } else {
                return None;
            }
        } else {
            self.particles[x.to_usize().unwrap()][y.to_usize().unwrap()] = None;
            return None;
        }
    }
    pub fn place_square<'a, T: PrimInt, E: Element<'a>>(
        &mut self,
        x: T,
        y: T,
        radius: T,
        element: &Option<Rc<(dyn Element<'static>)>>,
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
    pub fn draw<'a, T: PrimInt, E: Element<'a>>(
        &mut self,
        xc: T,
        yc: T,
        radius: T,
        element: Option<Rc<(dyn Element<'static>)>>,
    ) -> Vec<Option<Object<'static>>> {
        // println!("{:?} {:?}", xc.to_i32().unwrap(), yc.to_i32().unwrap());
        let mut placed = vec![];
        let x2;
        let y2;
        let x1;
        let y1;
        let dx;
        let dy;
        if xc.to_i32().unwrap() > self.last_dot.0 {
            x2 = xc.to_i32().unwrap();
            y2 = yc.to_i32().unwrap();
            x1 = self.last_dot.0;
            y1 = self.last_dot.1;
        } else {
            x1 = xc.to_i32().unwrap();
            y1 = yc.to_i32().unwrap();
            x2 = self.last_dot.0;
            y2 = self.last_dot.1;
        }

        dx = x2 - x1;
        dy = y2 - y1;
        if dx == 0 || dy == 0 {
            placed.append(&mut self.place_square::<T, E>(xc, yc, radius, &element));
        }

        // bresenham
        let mut x = x1;
        let mut y = y1;
        let mut err = dx / 2;
        while x < x2 {
            placed.append(&mut self.place_square::<i32, E>(
                x,
                y,
                radius.to_i32().unwrap(),
                &element,
            ));
            if err > 0 {
                y += 1;
                err -= dx;
            } else {
                err += dy;
            }
            x += 1;
        }

        self.last_dot = (xc.to_i32().unwrap(), yc.to_i32().unwrap());
        return placed;
    }
}
