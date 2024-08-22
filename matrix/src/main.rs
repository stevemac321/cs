fn main() {
    // Identity Matrix:
    // A special case of a diagonal matrix where all elements on the main diagonal are 1,
    // and all other elements are 0. It's denoted as I or I_n for an n×n matrix.
    // Example of a 5x5 identity matrix:
    let identity_matrix: [[i32; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1],
    ];

    /*In this tridiagonal matrix:
    The main diagonal contains the values 2, 3, 5, 6, and 1.
    The diagonal above the main diagonal (superdiagonal) contains 1, 4, 8, and 2.
    The diagonal below the main diagonal (subdiagonal) contains 1, 7, 9, and 3.
    All other elements are zero.
    */
    // Diagonal Matrix:
    // A square matrix where all elements outside the main diagonal are zero.
    // The main diagonal can contain any values, including zeros.
    // Example of a 5x5 diagonal matrix:
    let diagonal_matrix: [[i32; 5]; 5] = [
        [4, 0, 0, 0, 0],
        [0, 7, 0, 0, 0],
        [0, 0, 2, 0, 0],
        [0, 0, 0, 9, 0],
        [0, 0, 0, 0, 5],
    ];

    // Identity Matrix:
    // A special case of a diagonal matrix where all elements on the main diagonal are 1,
    // and all other elements are 0. It's denoted as I or I_n for an n×n matrix.
    // Example of a 5x5 identity matrix:
    let identity_matrix: [[i32; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1],
    ];

    // Upper Triangular Matrix:
    // In an upper triangular matrix U, u[i][j] = 0 for all i > j.
    // This means all elements below the main diagonal are zero,
    // while elements on and above the main diagonal can be non-zero.

    let upper_matrix: [[i32; 5]; 5] = [
        [1, 2, 3, 4, 5], // Row 0: All elements can be non-zero
        [0, 6, 7, 8, 9], // Row 1: First element must be zero
        [0, 0, 2, 3, 4], // Row 2: First two elements must be zero
        [0, 0, 0, 5, 6], // Row 3: First three elements must be zero
        [0, 0, 0, 0, 7], // Row 4: First four elements must be zero
    ];

    // This matrix conforms to the definition where u[i][j] = 0 for all i > j.
    // The main diagonal (where i == j) contains the values 1, 6, 2, 5, and 7.
    // All elements above the main diagonal (where i < j) can be non-zero.
    // All elements below the main diagonal (where i > j) are zero.

    // Lower Triangular Matrix:
    // In a lower triangular matrix L, l[i][j] = 0 for all i < j.
    // This means all elements above the main diagonal are zero,
    // while elements on and below the main diagonal can be non-zero.

    let lower_matrix: [[i32; 5]; 5] = [
        [1, 0, 0, 0, 0], // Row 0: Only the first element can be non-zero
        [2, 3, 0, 0, 0], // Row 1: First two elements can be non-zero
        [4, 5, 6, 0, 0], // Row 2: First three elements can be non-zero
        [7, 8, 9, 1, 0], // Row 3: First four elements can be non-zero
        [2, 3, 4, 5, 6], // Row 4: All elements can be non-zer// Permutation Matrix:
    ];

    // This matrix conforms to the definition where l[i][j] = 0 for all i < j.
    // The main diagonal (where i == j) contains the values 1, 3, 6, 1, and 6.
    // All elements below the main diagonal (where i > j) can be non-zero.
    // All elements above the main diagonal (where i < j) are zero.

    // A square matrix that has exactly one 1 in each row and each column, with 0s everywhere else.
    // It represents a permutation of the elements of a vector or the rows/columns of another matrix.
    // When multiplied with another matrix, it rearranges the rows or columns of that matrix.

    let permutation_matrix: [[i32; 5]; 5] = [
        [0, 1, 0, 0, 0], // Represents moving the 2nd element to the 1st position
        [0, 0, 0, 1, 0], // Represents moving the 4th element to the 2nd position
        [1, 0, 0, 0, 0], // Represents moving the 1st element to the 3rd position
        [0, 0, 0, 0, 1], // Represents moving the 5th element to the 4th position
        [0, 0, 1, 0, 0], // Represents moving the 3rd element to the 5th position
    ];

    // This permutation matrix represents the permutation [2, 4, 1, 5, 3]
    // It has exactly one 1 in each row and each column, with 0s everywhere else.

    // Symmetric Matrix:
    // A square matrix that is equal to its transpose.
    // In a symmetric matrix A, the element at position (i, j) is equal to the element at (j, i).
    // Formally, A[i][j] = A[j][i] for all i and j.

    let symmetric_matrix: [[i32; 5]; 5] = [
        [1, 3, -2, 4, 5],
        [3, 2, 0, -1, 6],
        [-2, 0, 4, 7, -3],
        [4, -1, 7, 3, 2],
        [5, 6, -3, 2, 8],
    ];

    // This symmetric matrix has the following properties:
    // 1. The main diagonal can contain any values (1, 2, 4, 3, 8 in this case).
    // 2. The upper triangular part (above the main diagonal) is a mirror image of the lower triangular part.
    // 3. A[i][j] = A[j][i] for all i and j. For example, A[0][1] = A[1][0] = 3, A[0][2] = A[2][0] = -2, etc.o
}
/*
*

### Overview of Matrix Operations

#### 1. **Scalar Multiplication (λ × A)**
   - **Definition**: Scalar multiplication involves multiplying each element of a matrix \( A \) by a scalar \( \lambda \) (a single number).
   - **Result**: The result is a new matrix of the same dimensions as \( A \), where each element has been scaled by \( \lambda \).
   - **Example**:
     \[
     \lambda = 3, \quad A = \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}
     \]
     \[
     \lambda A = \begin{pmatrix} 3 \times 1 & 3 \times 2 \\ 3 \times 3 & 3 \times 4 \end{pmatrix} = \begin{pmatrix} 3 & 6 \\ 9 & 12 \end{pmatrix}
     \]
   - **Purpose**: Scalar multiplication uniformly scales all elements in a matrix by a fixed factor.

#### 2. **Matrix Multiplication (A × B)**
   - **Definition**: Matrix multiplication involves multiplying two matrices \( A \) and \( B \) to produce a new matrix \( C \).
   - **Dimension Requirements**: For two matrices \( A \) and \( B \) to be multiplied, the number of columns in \( A \) must equal the number of rows in \( B \). The resulting matrix \( C \) will have dimensions determined by the number of rows in \( A \) and the number of columns in \( B \).
   - **Computation**:
     - Each element \( C[i][j] \) in the resulting matrix is computed by taking the dot product of the \( i \)th row of \( A \) and the \( j \)th column of \( B \).
     - The calculation for each element involves multiplying corresponding elements and summing the results.
   - **Example**:
     \[
     A = \begin{pmatrix} 1 & 2 & 3 \\ 4 & 5 & 6 \end{pmatrix}, \quad B = \begin{pmatrix} 7 & 8 & 9 & 10 \\ 11 & 12 & 13 & 14 \\ 15 & 16 & 17 & 18 \end{pmatrix}
     \]
     \[
     C = \begin{pmatrix} 74 & 80 & 86 & 92 \\ 173 & 188 & 203 & 218 \end{pmatrix}
     \]
   - **Non-Commutative**: Matrix multiplication is not commutative, meaning \( A \times B \) is not generally equal to \( B \times A \).
   - **Purpose**: Matrix multiplication is used to combine two matrices in a way that reflects complex interactions between their elements, often representing transformations or the combination of linear equations.

### Key Points:
- **Scalar Multiplication** results in a matrix where each element is independently scaled by a fixed factor.
- **Matrix Multiplication** produces a new matrix where each element is the sum of products of corresponding elements from the rows and columns of the original matrices.
- The result of matrix multiplication is a matrix, not a single summed value, with each element reflecting a distinct calculation.

*
* */
