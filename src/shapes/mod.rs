use raylib::prelude::*;

pub enum GeoShape {

    Circle,
    Square,
    Rectangle
}


pub struct Shape {

    pub shape: GeoShape,
    pub coordinates: Vector2,
    pub color: Color,
    pub size: Option<(i32, i32)>,
    pub radius: Option<f32>, //for circles.

}

pub fn default() -> Shape {

    Shape { 
            shape: (GeoShape::Circle), 
            coordinates: (Vector2 { x: (300.0), y: (255.0) }), 
            color: (Color::BLUE), 
            size: (Option::Some((30, 30))),
            radius: (Option::Some(30.0))
        }
}   


impl Shape {

    pub fn new(shape: GeoShape, coord: Vector2, shapeColor: Option<Color>, shapeSize: Option<(i32, i32)>, radius: Option<f32>) -> Self {
        Shape { 

            shape: shape,
            coordinates: coord, 
            color: match shapeColor {
                Some(c) => c,
                None => Color::BLUE
            },
            size: match shapeSize {
                Some(size) => Some((size.0, size.1)), 
                None => Some((15,15))
            },
            radius: match radius {

                Some(r) => Some(r),
                None => Some(15.0)
            }
                
        
        }           
    }   

  
}
