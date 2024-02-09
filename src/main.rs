mod matriz;

use ggez::{
    conf::{self, WindowMode, WindowSetup},
    event, graphics, Context, ContextBuilder, GameError, GameResult,
};
use matriz::Matriz;

const LINHAS: usize = 10;
const COLUNAS: usize = 10;
const TAMANHO: f32 = 20.0;

struct State {
    matriz: Matriz,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        for (i, linha) in self.matriz.grade.iter().enumerate() {
            for (j, &valor) in linha.iter().enumerate() {
                let cor = match valor {
                    0 => graphics::Color::RED,
                    1 => graphics::Color::BLUE,
                    _ => graphics::Color::BLACK,
                };
                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(i as f32 * TAMANHO, j as f32 * TAMANHO, TAMANHO, TAMANHO),
                    cor,
                )?;
                canvas.draw(&rect, graphics::DrawParam::default());
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() {
    let mut matriz = Matriz::new(LINHAS, COLUNAS);
    matriz.grade[2][2] = 1;
    matriz.grade[3][4] = 1;
    matriz.grade[4][3] = 1;

    let state = State { matriz: matriz };

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
