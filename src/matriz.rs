use core::fmt;

#[derive(Clone)]
pub struct Matriz {
    pub grade: Vec<Vec<u8>>,
}

impl Matriz {
    pub fn new(linhas: usize, colunas: usize) -> Matriz {
        Matriz {
            grade: vec![vec![0; colunas]; linhas],
        }
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
