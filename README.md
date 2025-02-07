# Rust Out-of-Bounds Vector Access

This repository demonstrates a common error in Rust: accessing an element in a vector using an index that is out of bounds.  The code in `bug.rs` will panic at runtime because it tries to access an index that doesn't exist.

The solution in `bugSolution.rs` shows how to prevent this error using safe indexing techniques.

## How to Run

1. Clone the repository
2. Navigate to the directory
3. Compile and run `bug.rs` (you'll see a panic)
4. Compile and run `bugSolution.rs` (this will execute without errors)
