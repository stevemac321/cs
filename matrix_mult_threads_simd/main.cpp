#include <iostream>
#include <vector>
// clang++ -mavx2 -std=c++23 -o mult main.cpp -pthread
// or mult.exe on windows

#include <chrono>
#include <iostream>
#include <thread>
#include <immintrin.h>  // For AVX intrinsics
#include <cstdlib>      // For rand()
#include <ctime>        // For seeding rand()

const size_t N = 1000;  // Matrix size 1000x1000
const size_t NUM_THREADS = std::thread::hardware_concurrency(); // Number of threads

// Function to generate a random matrix
std::vector<std::vector<float>> generate_random_matrix(size_t size) {
    std::vector<std::vector<float>> matrix(size, std::vector<float>(size));
    for (size_t i = 0; i < size; ++i) {
        for (size_t j = 0; j < size; ++j) {
            matrix[i][j] = static_cast<float>(rand()) / RAND_MAX; // Random float between 0 and 1
        }
    }
    return matrix;
}

// Function to multiply matrices using SIMD and multithreading
void multiply_matrices_simd(const std::vector<std::vector<float>>& A, 
                            const std::vector<std::vector<float>>& B, 
                            std::vector<std::vector<float>>& C, 
                            size_t start_row, size_t end_row) {
    for (size_t i = start_row; i < end_row; ++i) {
        for (size_t j = 0; j < N; ++j) {
            __m256 sum = _mm256_setzero_ps();  // Initialize sum as zero
            for (size_t k = 0; k < N; k += 8) {
                // Load 8 elements from row i of A and column j of B
                __m256 a_vals = _mm256_loadu_ps(&A[i][k]);
                __m256 b_vals = _mm256_loadu_ps(&B[k][j]);
                // Multiply and add to sum
                sum = _mm256_add_ps(sum, _mm256_mul_ps(a_vals, b_vals));
            }
            // Sum all 8 floats in the SIMD register and store the result
            float temp[8];
            _mm256_storeu_ps(temp, sum);
            C[i][j] = temp[0] + temp[1] + temp[2] + temp[3] + temp[4] + temp[5] + temp[6] + temp[7];
        }
    }
}

// Multithreaded matrix multiplication using SIMD
void threaded_matrix_multiplication(const std::vector<std::vector<float>>& A, 
                                    const std::vector<std::vector<float>>& B, 
                                    std::vector<std::vector<float>>& C) {
    std::vector<std::thread> threads;
    size_t rows_per_thread = N / NUM_THREADS;

    for (size_t t = 0; t < NUM_THREADS; ++t) {
        size_t start_row = t * rows_per_thread;
        size_t end_row = (t == NUM_THREADS - 1) ? N : start_row + rows_per_thread;
        threads.push_back(std::thread(multiply_matrices_simd, std::ref(A), std::ref(B), std::ref(C), start_row, end_row));
    }

    for (auto& thread : threads) {
        thread.join();  // Wait for all threads to finish
    }
}

int main() {
    srand(static_cast<unsigned>(time(0)));  // Seed the random number generator

    // Generate two random 1000x1000 matrices
    std::vector<std::vector<float>> A = generate_random_matrix(N);
    std::vector<std::vector<float>> B = generate_random_matrix(N);
    std::vector<std::vector<float>> C(N, std::vector<float>(N, 0.0f));  // Result matrix

	auto start = std::chrono::high_resolution_clock::now();
    // Perform multithreaded matrix multiplication using SIMD
    threaded_matrix_multiplication(A, B, C);

   auto end = std::chrono::high_resolution_clock::now();
std::chrono::duration<double> duration = end - start;
std::cout << "Time taken: " << duration.count() << " seconds" << std::endl;
}
