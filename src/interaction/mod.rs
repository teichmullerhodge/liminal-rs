use crate::{shapes::{self, GeoShape}, state::State};
use raylib::prelude::*;

use rand::seq::IndexedRandom;

pub fn 
generate_random_color() -> Color {
    //this is probably wrong...
    let mut randomThread =  rand::rng();
    let randomNumbers: Vec<u8> = (1..255).collect();
    let R = randomNumbers.choose(&mut randomThread).unwrap();
    let G = randomNumbers.choose(&mut randomThread).unwrap();
    let B = randomNumbers.choose(&mut randomThread).unwrap();
    let A = randomNumbers.choose(&mut randomThread).unwrap();

    Color { r: (*R), g: (*G), b: (*B), a: (*A) }
}


pub fn 
handle_movement(shapes: &mut Vec<shapes::Shape>, handler: &RaylibHandle, state: &mut State){

    if handler.is_key_down(KeyboardKey::KEY_RIGHT){          
        shapes[state.shapeRef].coordinates.x += 10.0;                       
    }
    if handler.is_key_down(KeyboardKey::KEY_LEFT){
            shapes[state.shapeRef].coordinates.x -= 10.0;                   
        }
    if handler.is_key_down(KeyboardKey::KEY_DOWN){   
            shapes[state.shapeRef].coordinates.y += 10.0;                       
        }
    if handler.is_key_down(KeyboardKey::KEY_UP){         
            shapes[state.shapeRef].coordinates.y -= 10.0;                
        }
}


pub fn
handle_window_interactions(handler: &RaylibHandle, state: &mut State){

    if handler.is_key_down(KeyboardKey::KEY_Q){
        state.quit = true;
    }
    
    //change background color 
    if handler.is_key_pressed(KeyboardKey::KEY_V) {
        state.backgroundColor = generate_random_color();
    }

}

pub fn 
handle_states(shapes: &mut Vec<shapes::Shape>, handler: &RaylibHandle, state: &mut State){

        if handler.is_key_pressed(KeyboardKey::KEY_N){

            state.geoShapeRef += 1;
            match state.geoShapeRef {
                0 => shapes[state.shapeRef].shape = GeoShape::Circle,
                1 => shapes[state.shapeRef].shape = GeoShape::Square,
                2 => shapes[state.shapeRef].shape = GeoShape::Rectangle,
                _ => {
                        state.geoShapeRef = 0;
                        shapes[state.shapeRef].shape = GeoShape::Circle;                        
                    }                
                }

            }

        // This should be refactored.
        if handler.is_key_down(KeyboardKey::KEY_FIVE) {

            match shapes[state.shapeRef].shape {
                
                GeoShape::Circle => {
                    if let Some(r) = shapes[state.shapeRef].radius {
                        shapes[state.shapeRef].radius = Some(r + 1.0);
                    }            
                }
                GeoShape::Square => {
                    if let Some((w,h)) = shapes[state.shapeRef].size {
                        shapes[state.shapeRef].size = Some((w + 1, h + 1));
                    }            
                },
                GeoShape::Rectangle => {
                    if let Some((w,h)) = shapes[state.shapeRef].size {
                        shapes[state.shapeRef].size = Some((w + 1, h + 1));
                    }            
                },
            }

        }
        if handler.is_key_down(KeyboardKey::KEY_TWO) {
            match shapes[state.shapeRef].shape {
                
                GeoShape::Circle => {
                    if let Some(r) = shapes[state.shapeRef].radius {
                        shapes[state.shapeRef].radius = Some(r - 1.0);
                    }            
                }
                GeoShape::Square => {
                    if let Some((w,h)) = shapes[state.shapeRef].size {
                        shapes[state.shapeRef].size = Some((w - 1, h - 1));
                    }            
                },
                GeoShape::Rectangle => {
                    if let Some((w,h)) = shapes[state.shapeRef].size {
                        shapes[state.shapeRef].size = Some((w - 1, h - 1));
                    }            
                },
            }
        }


        if handler.is_key_pressed(KeyboardKey::KEY_C) {
            shapes[state.shapeRef].color = generate_random_color();            
        }

        if handler.is_key_pressed(KeyboardKey::KEY_K) {

            shapes.push(shapes::default());
            state.shapeRef += 1;            
        }

        if handler.is_key_pressed(KeyboardKey::KEY_L) {
            shapes.pop();
            state.shapeRef -= 1;            
        }            
        
    }