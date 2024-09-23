## Overview 
This Rust project calculates the kernel and image of a homomorphism defined as: $\phi:\mathbb{Z}\to\mathbb{Z}_6$ where $\phi(n) = n \mod 6$. The kernel of a homomorphism consists of all elements in the domain that map to the identity element in the codomain. In this case, the kernel is the set of all integers $n$ such that $\phi(n) = 0 \mod 6$. The image is the set of all distinct outputs of the homomorphism for inputs from the domain.

## Features

- Computes the kernel of the homomorphism $\phi(n)$.
- Computes the image of the homomorphism.
- Outputs the kernel with ellipses to denote its infinite nature.
- Outputs the image in a sorted manner.

## Getting Started

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Running the Project

1. Clone the repository or create a new Rust project.
2. Replace the `main.rs` content with the code provided below.

```rust
use std::collections::HashSet;

// Define a homomorphism ϕ: Z → Z6, where ϕ(n) = n mod 6 (non-negative)
fn homomorphism(n: i32) -> i32 {
    let result = n % 6;
    if result < 0 {
        result + 6 // Ensure non-negative result
    } else {
        result
    }
}

// Find the kernel of the homomorphism (elements that map to 0 mod 6)
fn find_kernel(domain: &[i32]) -> Vec<i32> {
    let mut kernel: Vec<i32> = domain
        .iter()
        .filter(|&&n| homomorphism(n) == 0)
        .cloned()
        .collect();
    kernel.sort(); // Sort the kernel elements
    kernel
}

// Find the image of the homomorphism (all distinct values of ϕ(n) mod 6)
fn find_image(domain: &[i32]) -> Vec<i32> {
    let mut image: Vec<i32> = domain
        .iter()
        .map(|&n| homomorphism(n))
        .collect::<HashSet<_>>() // Collect distinct values
        .into_iter()
        .collect();
    image.sort(); // Sort the image values
    image
}

fn main() {
    // Define a larger domain, extending to include more multiples of 6
    let domain: Vec<i32> = (-100..=100).collect(); // Change the range as needed

    // Find kernel
    let kernel = find_kernel(&domain);
    let kernel_str: Vec<String> = kernel.iter().map(|n| n.to_string()).collect();
    println!("Kernel: {{..., {}, ...}}", kernel_str.join(", ")); // Show ellipses inside

    // Find image
    let image = find_image(&domain);
    println!("Image: {:?}", image); // Show sorted image
}

## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/cypriansakwa/Kernel_and_Image_of_a_Homomorphism_in_Rust.git
   cd Kernel_and_Image_of_a_Homomorphism_in_Rust
