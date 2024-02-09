use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics::Text,
    *,
};
mod matriz;
use matriz::Matriz;

struct State {
    matriz: Matriz,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.matriz = Matriz::new(10, 10);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 20.0, 20.0),
            graphics::Color::RED,
        )?;

        canvas.draw(&rect, graphics::DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() {
    let state = State {
        matriz: Matriz::new(10, 10),
    };

    let mut c = conf::Conf::new();
    c.window_mode = WindowMode {
        width: 200.0,
        height: 200.0,
        ..Default::default()
    };
    c.window_setup = WindowSetup {
        title: "Sand".to_owned(),
        ..Default::default()
    };

    let (ctx, event_loop) = ContextBuilder::new("Sand", "anthonyleier")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}

// fn main() {
//     let matriz = Matriz::new(10, 10);
//     println!("{:?}", matriz);
// }
