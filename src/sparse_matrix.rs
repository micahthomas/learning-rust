use std::cmp::Ordering;

#[derive(Clone)]
pub struct MatrixElement {
    row: i32,
    col: i32,
    value: f32,
}

impl Ord for MatrixElement {
    fn cmp(&self, other: &MatrixElement) -> Ordering {
        if self.row == other.row && self.col == other.col {
            return Ordering::Equal;
        } else if self.row < other.row || (self.row == other.row && self.col < other.col) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}

impl PartialOrd for MatrixElement {
    fn partial_cmp(&self, other: &MatrixElement) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for MatrixElement {
    fn eq(&self, other: &MatrixElement) -> bool {
        return self.row == other.row && self.col == other.col;
    }
}

impl Eq for MatrixElement {}

pub struct SparseMatrix {
    list: Vec<MatrixElement>,
    rows: i32,
    cols: i32,
    num_of_points: i32,
    sorted: bool,
    clean: bool,
}

impl SparseMatrix {
    pub fn new() -> SparseMatrix {
        return SparseMatrix {
            list: Vec::new(),
            rows: 0,
            cols: 0,
            num_of_points: 0,
            sorted: true,
            clean: true,
        };
    }

    fn add_point(&mut self, row: i32, col: i32, value: f32) {
        if row > self.rows {
            self.rows = row;
        }

        if col > self.cols {
            self.cols = col;
        }

        match self.list.last() {
            Some(last_element) => {
                if row < last_element.row || (row == last_element.row && col < last_element.col) {
                    self.sorted = false;
                } else if value == 0.0 {
                    self.clean = false;
                }
            }
            None => {}
        }

        self.list.push(MatrixElement {
            row: row,
            col: col,
            value: value,
        });

        self.num_of_points += 1;
    }

    fn ensure_sorted(&mut self) {
        if !self.sorted {
            self.list.sort();
            self.sorted = true;
        }
    }

    fn ensure_clean(&mut self) {
        if !self.clean {
            self.clean_zeros();
            self.clean = true;
        }
    }

    pub fn set_value_at_coordinate(&mut self, row: i32, col: i32, value: f32) {
        self.ensure_sorted();

        let seek = MatrixElement {
            row: row,
            col: col,
            value: 0.0,
        };

        match self.list.binary_search_by(|element| element.cmp(&seek)) {
            Ok(index) => self.list[index].value = value,
            Err(_) => self.add_point(row, col, value),
        }
    }

    pub fn get_value_at_coordinate(&mut self, row: i32, col: i32) -> f32 {
        self.ensure_sorted();

        let seek = MatrixElement {
            row: row,
            col: col,
            value: 0.0,
        };

        match self.list.binary_search_by(|element| element.cmp(&seek)) {
            Ok(index) => return self.list[index].value,
            Err(_) => return 0.0,
        }
    }

    pub fn get_number_of_points(&self) -> i32 {
        return self.num_of_points;
    }

    pub fn clean_zeros(&mut self) {
        let list_copy: Vec<MatrixElement> = self.list
            .clone()
            .into_iter()
            .filter(|element| {
                element.value != 0.0 || (element.row == self.rows && element.col == self.cols)
            })
            .collect();

        self.num_of_points = list_copy.len() as i32;
        self.list = list_copy;
    }

    pub fn matrix_multiplication(&mut self, matrix_a: &mut SparseMatrix) -> Option<SparseMatrix> {
        if self.rows != matrix_a.cols {
            return None;
        }

        self.ensure_clean();
        self.ensure_sorted();
        matrix_a.ensure_clean();
        matrix_a.ensure_sorted();


        let mut result = SparseMatrix::new();
        result.rows = self.rows;
        result.cols = matrix_a.cols;
        let mut rows: Vec<i32> = self.list.clone().into_iter().map(|element| element.row).collect();
        let mut cols: Vec<i32> =
            matrix_a.list.clone().into_iter().map(|element| element.col).collect();
        rows.dedup();
        cols.dedup();

        // O(x^2 * n^2), where x, n is num of elements in self and matrix_a respectively
        // much better than O(self.rows * matrix_a.cols * matrix_a.rows)
        for row in rows.iter() {
            for col in cols.iter() {
                let mut sum = 0.0;
                for cols_in_row_self in self.list
                    .clone()
                    .into_iter()
                    .filter(|element| element.row == *row) {
                    for rows_in_col_matrix_a in
                        matrix_a.list.clone().into_iter().filter(|element| element.col == *col) {
                        if cols_in_row_self.col == rows_in_col_matrix_a.row {
                            sum += cols_in_row_self.value * rows_in_col_matrix_a.value;
                        }
                    }
                }
                result.set_value_at_coordinate(*row, *col, sum);
            }
        }

        return Some(result);
    }

    pub fn print(&self) {
        for element in self.list.iter() {
            println!("Row: {}\t| Col: {}\t| Value: {}",
                     element.row,
                     element.col,
                     element.value);
        }
    }

    pub fn print_as_matrix(&mut self) {
        println!("{} {} - row col", self.rows, self.cols);
        for i in 1..self.rows + 1 {
            for j in 1..self.cols + 1 {
                print!("{}\t", self.get_value_at_coordinate(i, j));
            }
            println!("|");
        }
    }
}
