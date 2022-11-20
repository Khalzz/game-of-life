use std::{vec};
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};
mod cell;

struct Grid {
    height: usize,
    width: usize,
}

fn clean(mut _array:&mut Vec<Vec<cell::Box>>, grid_data:Grid) {
    for i in (0..grid_data.height).rev() {
        for j in (0..grid_data.width).rev() {            
            _array[i][j].state = false;
            _array[i][j].r = 50;
            _array[i][j].g = 50;
            _array[i][j].b = 50;            
        }
    }
}

fn main() {
    let mut running = false;

    let sdl_context = sdl2::init().unwrap();
    let video_susbsystem = sdl_context.video().unwrap();
    let _window = video_susbsystem.window("Basic grid", 1440, 900).position_centered().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut _canvas = _window.into_canvas().build().unwrap();

    const GRID_DATA:Grid = Grid {
        height: 60, // 16
        width: 97
    };
    
    let mut array = vec![vec![cell::Box::new(0 ,false, 0, 0, 50, 50, 50); GRID_DATA.width];GRID_DATA.height];
    
    let mut temp_array:Vec<Vec<cell::Box>>;
    let mut temp_x = 0;
    let mut temp_y = 0;

    for i in (0..GRID_DATA.height).rev() {
        for j in (0..GRID_DATA.width).rev() {     
            let mut _box = cell::Box {
                neighboors: 0,
                state: false,
                x: temp_x,
                y: temp_y,
                r: 0,
                g: 0,
                b: 0
            };           
            array[i][j] = _box;
            temp_x = 15* (j as i64) - 15;
            temp_y = 15* (i as i64);
        }
    }

    let mut is_running = true;
    while is_running {
        temp_array = array.clone();
        
        _canvas.set_draw_color(Color::RGB(150, 150, 150));
        _canvas.clear();
        
        // coloring the grid
        for i in (0..GRID_DATA.height).rev() {
            for j in (0..GRID_DATA.width).rev() {
                if (event_pump.mouse_state().x() > (array[i][j].x as i32) && event_pump.mouse_state().x() < (array[i][j].x as i32) + 15) && (event_pump.mouse_state().y() > (array[i][j].y as i32) && event_pump.mouse_state().y() < (array[i][j].y as i32) + 15) {
                    if array[i][j].state != true{
                        if event_pump.mouse_state().left(){
                            temp_array[i][j].state = true;
                        }
                        temp_array[i][j].r = 80; 
                        temp_array[i][j].g = 80;
                        temp_array[i][j].b = 80;
                    } else if array[i][j].state != false {
                        if event_pump.mouse_state().right(){
                            temp_array[i][j].state = false;
                        }
                    }
                } else {
                    if array[i][j].state == false && running == false{
                        temp_array[i][j].r = 50;
                        temp_array[i][j].g = 10;
                        temp_array[i][j].b = 10;
                    } else if running == true {
                        temp_array[i][j].r = 10;
                        temp_array[i][j].g = 10;
                        temp_array[i][j].b = 10;
                    }
                }
                if array[i][j].state == true {
                    array[i][j].r = 250;
                    array[i][j].g = 250;
                    array[i][j].b = 250;
                }
            }
        }
        // making the cells
        for i in (0..GRID_DATA.height).rev() {
            for j in (0..GRID_DATA.width - 1).rev() {
                _canvas.set_draw_color(Color::RGB(array[i][j].r, array[i][j].g, array[i][j].b)); // i change the color wich will define how the new square will be
                _canvas.fill_rect(Rect::new(array[i][j].x as i32,array[i][j].y as i32,14,14)).unwrap();
            }
        }

        // searching neighboors and changes
        for i in (0..GRID_DATA.height).rev() {
            for j in (0..GRID_DATA.width - 1).rev() {
                let neighboors = array[i][j].clone().count_neighboors(&array, i, j, running);

                if running == true {
                    if array[i][j].state == false && neighboors == 3 {
                        temp_array[i][j].state = true;
                    } 
                    if (array[i][j].state == true) && (neighboors == 2 || neighboors == 3) {
                        temp_array[i][j].state = true;
                    }
                    if (array[i][j].state == true) && (neighboors < 2 || neighboors > 3) {
                        temp_array[i][j].state = false;  
                    }
                }
            }
        }

        array = temp_array.clone(); // applying changes

        _canvas.present(); 
        for event in event_pump.poll_iter() { // we search for every event shown
            match event { // we search especificly events
                // in case we get a event quit or we press the escape key we close the window
                sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), .. }  => {
                    is_running = false;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Space), .. }  => {
                    if running == false {
                        running = true;
                    } else if running == true {
                        running = false;
                    }
                }, sdl2::event::Event::KeyDown { keycode: Some(Keycode::R), .. }  => {
                    running = false;
                    clean(&mut array, GRID_DATA);
                },
                _ => {} // in every other case we will do nothing
            }
        }
    }
}

