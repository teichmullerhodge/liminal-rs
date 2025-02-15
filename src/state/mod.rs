use raylib::{color::Color, drawing::RaylibDrawHandle, prelude::RaylibDraw};

use crate::shapes::{self, GeoShape};

pub struct State {
    pub shapeRef: usize,
    pub geoShapeRef: usize,
    pub backgroundColor: Color,
    pub quit: bool,
}

impl State {

    pub fn init() -> Self {
        State { shapeRef: 0, geoShapeRef: 0, quit: false, backgroundColor: Color::BLACK }
    }

    pub fn inc_shape_ref(&mut self) -> () {
        self.shapeRef += 1;
    }

    pub fn dec_shape_ref(&mut self) -> () {
        self.shapeRef -= 1;        
    }

    pub fn inc_geoshape_ref(&mut self) -> () {
        self.geoShapeRef += 1;
    }
    pub fn dec_geoshape_ref(&mut self) -> () {
        self.geoShapeRef -= 1;
    }
    
}

pub fn main_draw_geo_loop(draw:&mut RaylibDrawHandle<'_>, shapes: &mut Vec<shapes::Shape>) {

    for shape in shapes {
        draw_geoshape(&shape, draw);

    }

}

pub fn draw_geoshape(S: &shapes::Shape, draw: &mut RaylibDrawHandle) {
    match S.shape {
        GeoShape::Circle => {
            draw.draw_circle(S.coordinates.x as i32, S.coordinates.y as i32, S.radius.unwrap_or(15.0), S.color);
        }
        GeoShape::Square => {

            if let Some((w, h)) = S.size {
                draw.draw_rectangle(S.coordinates.x as i32, S.coordinates.y as i32, w,h, S.color);
            }
            else {
                draw.draw_rectangle(S.coordinates.x as i32, S.coordinates.y as i32, 30,30, S.color);
            }

           
        }
        GeoShape::Rectangle => {

            if let Some((w, h)) = S.size {
                draw.draw_rectangle(S.coordinates.x as i32, S.coordinates.y as i32, w,h, S.color);

            }
            else {
                draw.draw_rectangle(S.coordinates.x as i32, S.coordinates.y as i32, 30,20, S.color);
            }
        }

    }
}



