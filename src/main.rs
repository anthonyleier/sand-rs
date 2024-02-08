fn montar_grade(linhas: usize, colunas: usize) -> Vec<Vec<i32>> {
    let mut grade: Vec<Vec<i32>> = Vec::new();

    for _ in 0..linhas {
        let linha: Vec<i32> = vec![0; colunas];
        grade.push(linha);
    }

    return grade;
}

fn imprimir_grade(grade: Vec<Vec<i32>>) {
    for i in 0..grade.len() {
        for j in 0..grade.len() {
            print!("[{}]", grade[i][j]);
        }
        println!("");
    }
}

fn main() {
    let mut grade = montar_grade(5, 5);
    grade[3][4] = 1;
    imprimir_grade(grade);
}
