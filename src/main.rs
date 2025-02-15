#![allow(non_snake_case)]

use raylib::prelude::*;

pub mod interaction;
pub mod shapes;
pub mod state;


fn main() {

    let width = 800;
    let height = 600;

    let (mut appHandle, mainThread) = raylib::init()    
    .size(width, height)
    .title("Liminal-rs")
    .build();

    appHandle.set_target_fps(180);
    
    let mut appShapes: Vec<shapes::Shape> = Vec::new();
    let mut appState = state::State::init();

    appShapes.push(shapes::default());
    while appHandle.window_should_close() == false && appState.quit == false {

        interaction::handle_movement(&mut appShapes, &appHandle,  &mut appState);
        interaction::handle_states(&mut appShapes, &appHandle,  &mut appState);
        interaction::handle_window_interactions( &appHandle, &mut appState);

        let mut drawer = appHandle.begin_drawing(&mainThread);

        state::main_draw_geo_loop(&mut drawer, &mut appShapes);        
        drawer.clear_background(appState.backgroundColor);        
        
    }
    
    


}
