mod matriz;

use ggez::{
    conf::{self, WindowMode, WindowSetup},
    event::{self, MouseButton},
    graphics, Context, ContextBuilder, GameError, GameResult,
};
use matriz::Matriz;

const LINHAS: usize = 10;
const COLUNAS: usize = 10;
const TAMANHO: f32 = 20.0;

struct State {
    matriz: Matriz,
}
impl State {
    fn coordenadas_para_indice(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        let i = (x / TAMANHO) as usize;
        let j = (y / TAMANHO) as usize;

        if i < LINHAS && j < COLUNAS {
            Some((i, j))
        } else {
            None
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut proxima_matriz = self.matriz.clone();
        for (i, linha) in self.matriz.grade.iter().enumerate() {
            for (j, valor) in linha.iter().enumerate() {
                let pixel = self.matriz.grade[i][j];
                if pixel == 1 {
                    if let Some(resultado) = i.checked_sub(1) {
                        if i + 1 < LINHAS && j + 1 < COLUNAS {
                            let embaixo = self.matriz.grade[i][j + 1];
                            let embaixo_esquerda = self.matriz.grade[i - 1][j + 1];
                            let embaixo_direita = self.matriz.grade[i + 1][j + 1];

                            if embaixo == 0 {
                                proxima_matriz.grade[i][j] = 0;
                                proxima_matriz.grade[i][j + 1] = 1;
                            } else if embaixo_esquerda == 0 {
                                proxima_matriz.grade[i][j] = 0;
                                proxima_matriz.grade[i - 1][j + 1] = 1;
                            } else if embaixo_direita == 0 {
                                proxima_matriz.grade[i][j] = 0;
                                proxima_matriz.grade[i + 1][j + 1] = 1;
                            }
                        }
                    }
                }
            }
        }
        self.matriz = proxima_matriz;
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

        let mouse = ctx.mouse.position();
        if let Some((i, j)) = self.coordenadas_para_indice(mouse.x, mouse.y) {
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(i as f32 * TAMANHO, j as f32 * TAMANHO, TAMANHO, TAMANHO),
                graphics::Color::CYAN,
            )?;
            canvas.draw(&rect, graphics::DrawParam::default());
        }

        canvas.finish(ctx)?;
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            let mouse = ctx.mouse.position();
            if let Some((i, j)) = self.coordenadas_para_indice(mouse.x, mouse.y) {
                if self.matriz.grade[i][j] == 0 {
                    self.matriz.grade[i][j] = 1;
                } else {
                    self.matriz.grade[i][j] = 0;
                }
            }
        }
        Ok(())
    }
}

pub fn main() {
    let matriz = Matriz::new(LINHAS, COLUNAS);
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
