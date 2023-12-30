use nannou::{App, Frame};
use rand::{self, Rng};
use std::{
    borrow::Borrow,
    collections::{hash_map::Keys, HashMap, HashSet},
};

use crate::{BG_COLOR, FG_COLOR, LINE_WEIGHT, NODE_RADIUS};

#[derive(Debug)]
pub struct Graph {
    nodes: HashSet<String>,
    edges: Vec<(String, String)>,
    positions: HashMap<String, (f32, f32)>,
    width: u32,
    height: u32,
}

impl Graph {
    pub fn new(width: u32, height: u32) -> Self {
        let edges = Vec::new();
        let nodes = HashSet::new();
        let positions = HashMap::new();
        return Self {
            nodes,
            edges,
            positions,
            width,
            height,
        };
    }

    pub fn from_file() {
        todo!();
    }

    pub fn add_edge(&mut self, node_a: &str, node_b: &str) {
        self.edges.push((node_a.to_string(), node_b.to_string()));
        self.nodes.insert(node_a.to_string());
        self.nodes.insert(node_b.to_string());
    }

    pub fn set_node_position(&mut self, node: &str, position: (f32, f32)) {
        self.positions.insert(node.to_string(), position);
    }

    pub fn randomize_positions(&mut self) {
        let mut rng = rand::thread_rng();
        let width = self.width as i32;
        let height = self.height as i32;

        for node in self.nodes.borrow() {
            let x = rng.gen_range(-(width as f32 / 2.0)..width as f32 / 2.0);
            let y = rng.gen_range(-(height as f32 / 2.0)..height as f32 / 2.0);
            self.positions.insert(node.to_string(), (x, y));
        }
    }

    pub fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        draw.background().color(BG_COLOR);

        for node in self.nodes.borrow() {
            let (x, y) = self.positions.get(node).unwrap();
            draw.ellipse()
                .x_y(*x as f32, *y as f32)
                .w_h(NODE_RADIUS as f32, NODE_RADIUS as f32)
                .color(FG_COLOR);
        }

        for edge in self.edges.iter() {
            let a_pos = self.positions.get(&edge.0).unwrap();
            let b_pos = self.positions.get(&edge.1).unwrap();

            draw.line()
                .start([a_pos.0, a_pos.1].into())
                .end([b_pos.0, b_pos.1].into())
                .weight(LINE_WEIGHT)
                .color(FG_COLOR);
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
