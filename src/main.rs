use std::collections::HashSet;

// Define a homomorphism \u{03C6}: Z → Z6, where \u{03C6}(n) = n mod 6 (non-negative)
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

// Find the image of the homomorphism (all distinct values of \u{03C6}(n) mod 6)
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
    let domain: Vec<i32> = (-30..=30).collect(); // Change the range as needed

    // Find kernel
    let kernel = find_kernel(&domain);
    let kernel_str: Vec<String> = kernel.iter().map(|n| n.to_string()).collect();
    println!("Kernel: {{..., {}, ...}}", kernel_str.join(", ")); // Show ellipses inside

    // Find image
    let image = find_image(&domain);
    println!("Image: {:?}", image); // Show sorted image
}
