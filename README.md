## Overview 
This Rust project calculates the kernel and image of a homomorphism defined as: $\phi:\mathbb{Z}\to\mathbb{Z}_6$ where $\phi(n) = n \mod 6$. The kernel of a homomorphism consists of all elements in the domain that map to the identity element in the codomain. In this case, the kernel is the set of all integers $n$ such that $\phi(n) = 0 \mod 6$. The image is the set of all distinct outputs of the homomorphism for inputs from the domain.

## Features

- Computes the kernel of the homomorphism $\phi(n)$.
- Computes the image of the homomorphism.
- Outputs the kernel with ellipses to denote its infinite nature.
- Outputs the image in a sorted manner.

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
## Usage
- To use this code, you can clone the repository.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run
## Output
>```
>Kernel: {..., -30, -24, -18, -12, -6, 0, 6, 12, 18, 24, 30, ...}
>Image: [0, 1, 2, 3, 4, 5]

## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/Kernel_and_Image_of_a_Homomorphism_in_Rust.git
   cd Kernel_and_Image_of_a_Homomorphism_in_Rust
