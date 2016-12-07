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
    /// keeps track whether there are unnecessary zeros
    clean: bool,
}

impl SparseMatrix {
    /// Constructs a new `SparseMatrix`
    ///
    /// # Examples
    ///
    /// ```
    /// use graphs::sparse_matrix::SparseMatrix;
    ///
    /// let sparse_matrix = SparseMatrix::new();
    /// ```
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

    pub fn with_capcaity(size: i32) -> SparseMatrix {
        return SparseMatrix {
            list: Vec::with_capacity(size as usize),
            rows: 0,
            cols: 0,
            num_of_points: 0,
            sorted: true,
            clean: true,
        };
    }

    /// Adds a coordinate to the vecotr
    fn add_point(&mut self, row: i32, col: i32, value: f32) {
        if row > self.rows {
            self.rows = row;
        }

        if col > self.cols {
            self.cols = col;
        }

        match self.list.last() {
            /// if there is a last_element, might have to change sorted or clean flags
            Some(last_element) => {
                if row < last_element.row || (row == last_element.row && col < last_element.col) {
                    self.sorted = false;
                } else if value == 0.0 {
                    self.clean = false;
                }
            }
            /// if there is no element, no need to change the sorted or clean flags
            None => {}
        }

        self.list.push(MatrixElement {
            row: row,
            col: col,
            value: value,
        });

        self.num_of_points += 1;
    }

    /// makes sure that the vector of matrix elements are sorted
    fn ensure_sorted(&mut self) {
        if !self.sorted {
            self.list.sort();
            self.sorted = true;
        }
    }

    /// makes sure that there are no unnecessary zeros
    fn ensure_clean(&mut self) {
        if !self.clean {
            self.clean_zeros();
            self.clean = true;
        }
    }

    /// Sets the value at a row and column to the specified value
    ///
    /// Changes the value of an existing point in the matrix if a corresponding
    /// point is found, otherwise adds a new point to the matrix
    ///
    /// # Examples
    ///
    /// ```
    /// use graphs::sparse_matrix::SparseMatrix;
    ///
    /// let mut sparse_matrix = SparseMatrix::new();
    /// sparse_matrix.set_value_at_coordinate(1, 1, 2.2);
    /// ```
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

    /// Gets the value at a row and column
    ///
    /// Gets the value of a row and column, if no corresponding entry is found,
    /// the function will return zero
    ///
    /// # Examples
    ///
    /// ```
    /// use graphs::sparse_matrix::SparseMatrix;
    ///
    /// let mut sparse_matrix = SparseMatrix::new();
    /// sparse_matrix.set_value_at_coordinate(1, 1, 2.2);
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

    fn get_list_of_rows(&self) -> Vec<i32> {
        let mut rows: Vec<i32> = self.list.clone().into_iter().map(|element| element.row).collect();
        rows.dedup();
        return rows;
    }

    fn get_list_of_cols(&self) -> Vec<i32> {
        let mut cols: Vec<i32> = self.list.clone().into_iter().map(|element| element.col).collect();
        cols.dedup();
        return cols;
    }

    fn get_row(&self, row: i32) -> Vec<MatrixElement> {
        return self.list
            .clone()
            .into_iter()
            .filter(|element| element.row == row)
            .collect();
    }

    fn get_col(&self, col: i32) -> Vec<MatrixElement> {
        return self.list
            .clone()
            .into_iter()
            .filter(|element| element.col == col)
            .collect();

    }

    pub fn matrix_multiplication(&mut self, matrix_a: &mut SparseMatrix) -> Option<SparseMatrix> {
        if self.rows != matrix_a.cols {
            return None;
        }

        self.ensure_clean();
        self.ensure_sorted();
        matrix_a.ensure_clean();
        matrix_a.ensure_sorted();


        let mut result = SparseMatrix::with_capcaity((matrix_a.get_number_of_points() +
                                                      self.get_number_of_points()) /
                                                     2);
        result.rows = self.rows;
        result.cols = matrix_a.cols;

        let rows = self.get_list_of_rows();
        let cols = matrix_a.get_list_of_cols();

        // O(x^2 * n^2), where x, n is num of elements in self and matrix_a respectively
        // much better than O(self.rows * matrix_a.cols * matrix_a.rows)
        for row in &rows {
            for col in &cols {
                let mut sum = 0.0;
                let self_row = self.get_row(*row);
                let matrix_a_col = matrix_a.get_col(*col);
                for self_element in &self_row {
                    for matrix_a_element in &matrix_a_col {
                        if self_element.col == matrix_a_element.row {
                            sum += self_element.value * matrix_a_element.value;
                        }
                    }
                }
                if sum != 0.0 {
                    result.set_value_at_coordinate(*row, *col, sum);
                }
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
