pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::new();

        if row_count == 0 {
            return PascalsTriangle { triangle: triangle };
        }

        for x in 1..row_count + 1 {
            let mut dvec: Vec<u32> = Vec::new();
            for _ in 0..x {
                dvec.push(1);
            }
            triangle.push(dvec);
        }

        for x in 0..row_count {
            if triangle[x as usize].len() <= 2 {
                continue;
            } else {
                let num_of_changes = x - 1;
                for s in 1..num_of_changes + 1 {
                    triangle[x as usize][s as usize] = triangle[x as usize - 1][s as usize - 1] +
                        triangle[x as usize - 1][s as usize];
                }
            }
        }
        PascalsTriangle { triangle: triangle }

    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
