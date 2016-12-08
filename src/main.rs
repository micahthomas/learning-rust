mod sparse_matrix;

fn main() {
    let mut matrix_a = sparse_matrix::SparseMatrix::new();
    let mut matrix_b = sparse_matrix::SparseMatrix::new();

    for i in 1..1001 {
        matrix_a.set_value_at_coordinate(i, i, 1.0);
        matrix_b.set_value_at_coordinate(i, i, 1.0);
    }

    let matrix_result = matrix_a.matrix_multiplication(&mut matrix_b)
        .expect("matrix multiplcation not possible");
    // matrix_result.print_as_matrix();
    // matrix_result.print();
    println!("Number of Matrix Elements: {}",
             matrix_result.get_number_of_points());
}