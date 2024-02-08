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

fn main() {
    let mut grade = montar_grade();
    grade[3][4] = 1;
    imprimir_grade(grade);
}
