pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut main_triangle: Vec<Vec<u32>> = Vec::new();
        for _ in 1..=self.0 {
            let mut row = Vec::new();
            match main_triangle.last() {
                None => {
                    row.push(1);
                }
                Some(prev_row) => {
                    for i in 0..=prev_row.len() {
                        if i == 0 || i == prev_row.len() {
                            row.push(1)
                        } else {
                            row.push(prev_row[i - 1] + prev_row[i])
                        }
                    }
                }
            };
            main_triangle.push(row);
        }
        main_triangle
    }
}
