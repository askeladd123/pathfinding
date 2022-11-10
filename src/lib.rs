#![allow(unused_variables)]
#![allow(dead_code)]

mod search;
mod grid;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::grid::{Grid, Tile};
use crate::search::{astar_visual, VisualResult};

#[wasm_bindgen]
pub struct Data {
    dos: web_sys::Document,
    ctx: web_sys::CanvasRenderingContext2d,
    grid_offscreen: Grid,
    grid_onscreen: Grid,
    step: u32,
    control_panel: web_sys::Element,
    wall_indices: Vec<usize>,
}

#[wasm_bindgen]
impl Data {
    pub fn new()->Self{
    
        console_log::init().unwrap();
    
        let window = web_sys::window().unwrap();
        let dos = window.document().unwrap();
        let canvas = dos
            .get_element_by_id("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ()).unwrap();
    
        let grid_first = Grid::new_random(
            (64, 32),
            (
                canvas.width() as f32,
                canvas.height() as f32,
            ),
        );
        
        Self {
            control_panel: dos.get_element_by_id("control panel").unwrap(),
            dos,
            ctx: canvas
                .get_context("2d")
                .unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap(),
            grid_onscreen: grid_first.clone(),
            grid_offscreen: grid_first,
            step: 0,
            wall_indices: Vec::new(),
        }
    }
    
    fn print(& self, output: & str){
        self.control_panel.set_text_content(Some(output));
    }
    
    pub fn tick(&mut self) {
        
        if let Some(i) = self.wall_indices.pop(){
            self.grid_onscreen.set_tile(i, Tile::Wall);
            if self.wall_indices.is_empty(){
                self.grid_offscreen = self.grid_onscreen.clone();
            }
        }
        else {
            match astar_visual(&self.grid_offscreen, self.step, search::cost_euclidian) {
                VisualResult::Found { path } => {
                    for pos in path.into_iter() {
                        self.grid_onscreen.set_tile(pos, Tile::Path);
                    }
                    self.grid_onscreen.set_all(Tile::Empty);
                    self.wall_indices.append(&mut self.grid_onscreen.shuffle_visual());
                    self.step = 0;
                },
                VisualResult::NotFound { queued, head } => {
                    self.grid_onscreen.set_tile(head, Tile::Head);
                    for pos in queued.into_iter() {
                        self.grid_onscreen.set_tile(pos, Tile::Queued);
                    }
                    self.step += 1;
                },
                VisualResult::Impossible => {
                    self.grid_onscreen.set_all(Tile::Empty);
                    self.wall_indices.append(&mut self.grid_onscreen.shuffle_visual());
                    self.step = 0;
                },
            }
        }
    
        self.print(& format!("steps: {}", self.step));
        self.grid_onscreen.draw(& self.ctx);
    }
}