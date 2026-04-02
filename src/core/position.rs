#[derive(Debug, Clone)]
pub struct OktoPosition {
    pub file: Option<usize>,
    pub line: usize,
    pub column: Option<usize>
}

impl OktoPosition {
    pub fn new(file: Option<usize>, line: usize, column: Option<usize>) -> Self {
        Self { file, line, column }
    }
}