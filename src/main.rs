use nannou::prelude::*;

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;
const NODE_RADIUS: usize = 10;
const LINE_WEIGHT: f32 = 1.0;
const BG_COLOR: Rgb<u8> = BLACK;
const FG_COLOR: Rgb<u8> = PLUM;
const FFG_COLOR: Rgb<u8> = RED;

mod graph;
use graph::Graph;

struct Model {
    playing: bool,
    graph: Graph,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size_pixels(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();
    // app.set_loop_mode(LoopMode::rate_fps(2.0));

    let mut graph = Graph::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    graph.add_edge("1", "2");
    graph.add_edge("1", "3");
    graph.add_edge("1", "4");
    graph.add_edge("2", "4");
    graph.randomize_positions();

    Model {
        playing: false,
        graph,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.playing {
        // update
        // todo!();
        model.graph.randomize_positions();
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(event),
        } => match event {
            KeyPressed(Key::Space) => {
                model.playing = !model.playing;
            }
            _ => {}
        },
        // Event::DeviceEvent(_, _) => todo!(),
        // Event::Update(_) => todo!(),
        // Event::Suspended => todo!(),
        // Event::Resumed => todo!(),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    model.graph.draw(app, &frame);
}

fn main() {
    nannou::app(model).update(update).event(event).run();
}
