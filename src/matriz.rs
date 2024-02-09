pub struct Matriz {
    data: Vec<Vec<u8>>,
}

impl Matriz {
    pub fn new(linhas: usize, colunas: usize) -> Matriz {
        Matriz {
            data: vec![vec![0; colunas]; linhas],
        }
    }

    // ... outras funções para acessar e modificar a matriz
}
