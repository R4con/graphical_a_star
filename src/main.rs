extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
mod a_star;
//use rand::Rng;

pub struct App<'a> {    //? Variables used in App go here
    gl: GlGraphics, //  OpenGL drawing backend
    tile_size: f64,
    spacing_size: f64,
    map: &'a mut[&'a mut[bool]],
    final_path: Vec<(i32, i32)>
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let tile_size = self.tile_size;
        let spacing_size = self.spacing_size;
        let map = &self.map;
        let final_path = self.final_path.clone();

        const COLOR_TRUE: [f32;4] = [0.9, 0.9, 0.9, 1.0];
        const COLOR_FALSE: [f32;4] = [0.36, 0.36, 0.36, 1.0];
        const COLOR_FINAL_PATH: [f32; 4] = [0.1, 0.6, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear([0.25, 0.25, 0.25, 1.0], gl);

            for (y, map_slice) in map.iter().enumerate() {
                for (x, &tile) in map_slice.iter().enumerate() {
                    if final_path.contains(&(x as i32, y as i32)) {
                        rectangle(
                            COLOR_FINAL_PATH,
                            rectangle::square(0.0, 0.0, tile_size),
                            c.transform.trans(x as f64 * (tile_size + spacing_size) + spacing_size, y as f64 * (tile_size + spacing_size) + spacing_size), gl);
                    }
                    else if tile == true {
                        rectangle(
                            COLOR_TRUE,
                            rectangle::square(0.0, 0.0, tile_size),
                            c.transform.trans(x as f64 * (tile_size + spacing_size) + spacing_size, y as f64 * (tile_size + spacing_size) + spacing_size), gl);
                    }
                    else {
                        rectangle(
                            COLOR_FALSE,
                            rectangle::square(0.0, 0.0, tile_size),
                            c.transform.trans(x as f64* (tile_size + spacing_size) + spacing_size, y as f64* (tile_size + spacing_size) + spacing_size), gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        //change the color of the next square in the path
    }

    fn process_mouse(&mut self, cursor: [f64; 2]) {
        //get the tile the cursor clicked on:
        let x_tile = f64::floor((cursor[0] - self.spacing_size)  / (self. tile_size + self.spacing_size));
        let y_tile = f64::floor((cursor[1] - self.spacing_size)  / (self. tile_size + self.spacing_size));
        println!("x:{}, y:{}", x_tile, y_tile);
        
        //change value of that tile:
        self.map[y_tile as usize][x_tile as usize] = !self.map[y_tile as usize][x_tile as usize];       // ! cannot change value of the reference ?? try dereferencing ?
    }

    fn process_recalc(closed_list: &a_star::Node, final_path: &a_star::Node, start_node: &a_star::Node, target_node: &a_star::Node, NEW_MAP: &mut[&mut[bool]], open_list: &mut HashMap<(i32, i32), Node>, closed_list: &mut HashMap<(i32, i32), Node>) {
        //recalculate the path with the updated map
        closed_list = a_star::Node::loop_neighbours(&start_node, &target_node, NEW_MAP, &mut open_list);
        final_path = a_star::Node::get_path(&start_node, &target_node, &closed_list);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working
    let opengl = OpenGL::V3_2;
    let mut cursor = [0.0, 0.0];

    const TILE_SIZE: f64 = 50.0;
    const SPACING_SIZE: f64 = 5.0;

    let NEW_MAP: &mut[&mut[bool]] = &mut[
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
        &mut[false,false,false,false,false,false,false,false,false,false],
    ];

    //do a-star calculation:
    let start_node:  a_star::Node = a_star::Node::new(0, 0, 0);
    let target_node: a_star::Node = a_star::Node::new(9, 9, 0);

    //init open list with neighbours from the starting node
    let mut open_list = a_star::Node::init_neighbours(&start_node, &target_node, NEW_MAP);

    //cycle threw every neighbour, and go to the neighbour with lowest f_cost, until  target_node is reached //! return Option with Node, if no Path could be found
    let mut closed_list = a_star::Node::loop_neighbours(&start_node, &target_node, NEW_MAP, &mut open_list);
    
    let mut final_path = a_star::Node::get_path(&start_node, &target_node, &closed_list);

    //Output the calculated stuff
    //Create an Glutin window
    let mut window: Window = WindowSettings::new(
        "spinning_square",
        [NEW_MAP[0].len() as f64 * (TILE_SIZE + SPACING_SIZE) + SPACING_SIZE, NEW_MAP.len() as f64 * (TILE_SIZE + SPACING_SIZE) + SPACING_SIZE]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    // Create a new game and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
        map: NEW_MAP,
        tile_size: TILE_SIZE,
        spacing_size: SPACING_SIZE,
        final_path: final_path,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(button)) = e.press_args() {
            app.process_mouse(cursor);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::C {
                println!("recalculation");
                app.process_recalc(&mut closed_list, &mut final_path, &start_node, &target_node, NEW_MAP, &mut open_list, &closed_list);
            }
        };

        e.mouse_cursor(|pos| {
            cursor = pos;
        });

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}