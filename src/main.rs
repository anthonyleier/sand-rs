mod cor;
mod matriz;

use cor::gerar_cor;
use ggez::{
    conf::{self, WindowMode, WindowSetup},
    event::{self, MouseButton},
    graphics, Context, ContextBuilder, GameError, GameResult,
};
use matriz::Matriz;

const LINHAS: usize = 50;
const COLUNAS: usize = 50;
const TAMANHO: f32 = 10.0;

struct State {
    matriz: Matriz,
    hue: u8,
    gerar_areia: bool,
}
impl State {
    fn aumentar_hue(&mut self) {
        if let Some(resultado) = self.hue.checked_add(5) {
            self.hue = resultado;
        } else {
            self.hue = 1;
        }
    }
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
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.gerar_areia {
            let mouse = ctx.mouse.position();
            if let Some((mouse_i, mouse_j)) = self.coordenadas_para_indice(mouse.x, mouse.y) {
                let tamanho_matriz_areia = 3.0;
                let limite = (tamanho_matriz_areia / 2.0 as f32).floor() as i32;

                for i in -limite..=limite {
                    for j in -limite..=limite {
                        let areia_i = mouse_i as i32 + i;
                        let areia_j = mouse_j as i32 + j;
                        self.matriz.grade[areia_i as usize][areia_j as usize] = self.hue;
                    }
                }
                self.aumentar_hue();
            }
        }
        self.matriz = self.matriz.atualizar();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        for (i, linha) in self.matriz.grade.iter().enumerate() {
            for (j, &valor) in linha.iter().enumerate() {
                if valor > 0 {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            i as f32 * TAMANHO,
                            j as f32 * TAMANHO,
                            TAMANHO,
                            TAMANHO,
                        ),
                        gerar_cor(valor),
                    )?;
                    canvas.draw(&rect, graphics::DrawParam::default());
                }
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            self.gerar_areia = true;
        }
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            self.gerar_areia = false;
        }
        Ok(())
    }
}

pub fn main() {
    let matriz = Matriz::new(LINHAS, COLUNAS);
    let state = State {
        matriz: matriz,
        hue: 1,
        gerar_areia: false,
    };

    let mut c = conf::Conf::new();
    c.window_mode = WindowMode {
        width: 500.0,
        height: 500.0,
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
