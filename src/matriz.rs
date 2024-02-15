use core::fmt;

#[derive(Clone)]
pub struct Matriz {
    pub linhas: usize,
    pub colunas: usize,
    pub tamanho: f32,
    pub grade: Vec<Vec<u8>>,
}

impl Matriz {
    pub fn new(linhas: usize, colunas: usize, tamanho: f32) -> Matriz {
        Matriz {
            linhas: linhas,
            colunas: colunas,
            tamanho: tamanho,
            grade: vec![vec![0; colunas]; linhas],
        }
    }

    pub fn atualizar(&self) -> Matriz {
        let mut proxima_matriz = self.clone();
        for (i, linha) in self.grade.iter().enumerate() {
            for (j, valor) in linha.iter().enumerate() {
                let pixel = self.grade[i][j];
                if pixel > 0 {
                    if j + 1 < self.colunas {
                        let mut pode_ir_esquerda = true;
                        let mut pode_ir_direita = true;
                        let mut embaixo_esquerda = 1;
                        let mut embaixo_direita = 1;

                        // Caso normal
                        if i + 1 < self.linhas {
                            pode_ir_direita = true;
                            pode_ir_esquerda = true;
                        }

                        // Regra do canto esquerdo
                        if i == 0 {
                            pode_ir_esquerda = false;
                        }

                        // Regra do canto direito
                        if i + 1 == self.linhas {
                            pode_ir_direita = false;
                        }

                        let embaixo = self.grade[i][j + 1];
                        if pode_ir_esquerda {
                            embaixo_esquerda = self.grade[i - 1][j + 1];
                        }
                        if pode_ir_direita {
                            embaixo_direita = self.grade[i + 1][j + 1];
                        }

                        if embaixo == 0 {
                            proxima_matriz.grade[i][j] = 0;
                            proxima_matriz.grade[i][j + 1] = pixel;
                        } else if pode_ir_esquerda && embaixo_esquerda == 0 {
                            proxima_matriz.grade[i][j] = 0;
                            proxima_matriz.grade[i - 1][j + 1] = pixel;
                        } else if pode_ir_direita && embaixo_direita == 0 {
                            proxima_matriz.grade[i][j] = 0;
                            proxima_matriz.grade[i + 1][j + 1] = pixel;
                        }
                    }
                }
            }
        }
        return proxima_matriz;
    }
}

impl fmt::Debug for Matriz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for linha in &self.grade {
            for &valor in linha {
                write!(f, "[{}]", valor)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
