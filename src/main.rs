mod sparse_matrix;

fn main() {
    let mut matrix_a = sparse_matrix::SparseMatrix::new();
    matrix_a.set_value_at_coordinate(1, 1, 0.0);
    matrix_a.set_value_at_coordinate(2, 2, 2.5);
    matrix_a.set_value_at_coordinate(1, 1, 0.0);
    println!("{}", matrix_a.get_number_of_points());
    matrix_a.remove_zeros();
    println!("{}", matrix_a.get_value_at_coordinate(2, 2));
    println!("{}", matrix_a.get_number_of_points());
}