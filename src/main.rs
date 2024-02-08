use speedy2d::{
    color::Color,
    dimen::{Vec2, Vector2},
    window::WindowHandler,
    Window,
};

const LINHAS: usize = 5;
const COLUNAS: usize = 10;

fn montar_grade() -> [[i32; COLUNAS]; LINHAS] {
    let grade: [[i32; COLUNAS]; LINHAS] = [[0; COLUNAS]; LINHAS];
    grade
}

fn imprimir_grade(grade: [[i32; COLUNAS]; LINHAS]) {
    for i in 0..LINHAS {
        for j in 0..COLUNAS {
            print!("[{}]", grade[i][j]);
        }
        println!("");
    }
}

struct MyWindowHandler {}
impl WindowHandler for MyWindowHandler {
    fn on_draw(
        &mut self,
        helper: &mut speedy2d::window::WindowHelper<()>,
        graphics: &mut speedy2d::Graphics2D,
    ) {
        let x = Vec2::new(0.0, 0.0);
        let y = Vec2::new(100.0, 0.0);
        let z = Vec2::new(100.0, 100.0);
        let w = Vec2::new(0.0, 100.0);
        graphics.draw_quad([x, y, z, w], Color::RED);
    }
}

fn main() {
    let mut grade = montar_grade();
    grade[3][4] = 1;
    imprimir_grade(grade);

    let game = Window::new_centered("Sand", (800, 480)).unwrap();
    let window = MyWindowHandler {};
    game.run_loop(window);
}
