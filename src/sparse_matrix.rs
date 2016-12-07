#[derive(Clone)]
pub struct MatrixElement {
    row: i32,
    col: i32,
    value: f32,
}

pub struct SparseMatrix {
    list: Vec<MatrixElement>,
    rows: i32,
    cols: i32,
    num_of_points: i32,
}

impl SparseMatrix {
    pub fn new() -> SparseMatrix {
        return SparseMatrix {
            list: Vec::new(),
            rows: 0,
            cols: 0,
            num_of_points: 0,
        };
    }

    fn add_point(&mut self, row: i32, col: i32, value: f32) {
        if row > self.rows {
            self.rows = row;
        }

        if col > self.cols {
            self.cols = col;
        }

        self.list.push(MatrixElement {
            row: row,
            col: col,
            value: value,
        });

        self.num_of_points += 1;
    }

    pub fn set_value_at_coordinate(&mut self, row: i32, col: i32, value: f32) {
        for element in self.list.iter_mut() {
            if element.row == row && element.col == col {
                element.value = value;
                return;
            }
        }

        self.add_point(row, col, value);
    }

    pub fn get_value_at_coordinate(&self, row: i32, col: i32) -> f32 {
        for element in self.list.iter() {
            if element.row == row && element.col == col {
                return element.value;
            }
        }

        return 0.0;
    }

    pub fn get_number_of_points(&self) -> i32 {
        return self.num_of_points;
    }

    pub fn remove_zeros(&mut self) {
        // let mut list_copy = self.list.clone();
        // let mut offset = 0;

        // for (i, element) in self.list.iter().enumerate() {
        //     if element.value == 0.0 {
        //         list_copy.swap_remove(i - offset);
        //         offset += 1;
        //         self.num_of_points -= 1;
        //     }
        // }

        // This implementation seems safer, need to investigate if the above implementation
        // would cause data races
        let list_copy: Vec<MatrixElement> = self.list.clone().into_iter().filter(|element| element.value != 0.0).collect();
        self.num_of_points = list_copy.len() as i32;

        self.list = list_copy;
    }
}
