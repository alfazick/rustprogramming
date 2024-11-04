use std::arch::x86_64::*;
use std::time::Instant;
use std::alloc::{alloc, dealloc, Layout};

// Function to create an aligned vector
fn create_aligned_vec(size: usize, value: f32) -> Vec<f32> {
    let align = 32; // 32-byte alignment for AVX
    let layout = Layout::from_size_align(size * std::mem::size_of::<f32>(), align).unwrap();
    unsafe {
        let ptr = alloc(layout) as *mut f32;
        for i in 0..size {
            *ptr.add(i) = value;
        }
        Vec::from_raw_parts(ptr, size, size)
    }
}

fn main() {
    let size = 1024 * 1024; // 1M elements
    let iterations = 100;
    
    println!("Vector Addition Benchmark");
    println!("Array size: {} elements", size);
    println!("Iterations: {}\n", iterations);

    // 1. Regular Addition
    let a1 = vec![1.0_f32; size];
    let b1 = vec![2.0_f32; size];
    let mut result1 = vec![0.0_f32; size];
    
    let mut regular_time = 0.0;
    for _ in 0..iterations {
        let start = Instant::now();
        for i in 0..size {
            result1[i] = a1[i] + b1[i];
        }
        regular_time += start.elapsed().as_secs_f64() * 1000.0;
    }

    // 2. SIMD Addition (unaligned)
    let a2 = vec![1.0_f32; size];
    let b2 = vec![2.0_f32; size];
    let mut result2 = vec![0.0_f32; size];
    
    let mut simd_time = 0.0;
    for _ in 0..iterations {
        let start = Instant::now();
        unsafe {
            let chunks = size / 8;
            for i in 0..chunks {
                let idx = i * 8;
                let a_vec = _mm256_loadu_ps(a2.as_ptr().add(idx));
                let b_vec = _mm256_loadu_ps(b2.as_ptr().add(idx));
                let sum = _mm256_add_ps(a_vec, b_vec);
                _mm256_storeu_ps(result2.as_mut_ptr().add(idx), sum);
            }
            // Handle remaining elements
            for i in (chunks * 8)..size {
                result2[i] = a2[i] + b2[i];
            }
        }
        simd_time += start.elapsed().as_secs_f64() * 1000.0;
    }

    // 3. Aligned SIMD Addition
    let a3 = create_aligned_vec(size, 1.0);
    let b3 = create_aligned_vec(size, 2.0);
    let mut result3 = create_aligned_vec(size, 0.0);
    
    let mut aligned_time = 0.0;
    for _ in 0..iterations {
        let start = Instant::now();
        unsafe {
            let chunks = size / 8;
            for i in 0..chunks {
                let idx = i * 8;
                let a_vec = _mm256_load_ps(a3.as_ptr().add(idx));
                let b_vec = _mm256_load_ps(b3.as_ptr().add(idx));
                let sum = _mm256_add_ps(a_vec, b_vec);
                _mm256_store_ps(result3.as_mut_ptr().add(idx), sum);
            }
            // Handle remaining elements
            for i in (chunks * 8)..size {
                result3[i] = a3[i] + b3[i];
            }
        }
        aligned_time += start.elapsed().as_secs_f64() * 1000.0;
    }

    // Calculate averages
    let avg_regular = regular_time / iterations as f64;
    let avg_simd = simd_time / iterations as f64;
    let avg_aligned = aligned_time / iterations as f64;

    // Print results
    println!("1. Regular Addition     : {:.3} ms", avg_regular);
    println!("2. SIMD Addition       : {:.3} ms ({:.2}x speedup)", 
             avg_simd, avg_regular/avg_simd);
    println!("3. Aligned SIMD        : {:.3} ms ({:.2}x speedup)", 
             avg_aligned, avg_regular/avg_aligned);
    
    // Verify results are correct
    println!("\nVerification:");
    println!("Regular sum : {}", result1.iter().sum::<f32>());
    println!("SIMD sum    : {}", result2.iter().sum::<f32>());
    println!("Aligned sum : {}", result3.iter().sum::<f32>());

    // Print memory alignment info
    println!("\nMemory Alignment:");
    println!("Regular: {:p} (mod 32 = {})", a1.as_ptr(), 
             (a1.as_ptr() as usize) % 32);
    println!("SIMD   : {:p} (mod 32 = {})", a2.as_ptr(), 
             (a2.as_ptr() as usize) % 32);
    println!("Aligned: {:p} (mod 32 = {})", a3.as_ptr(), 
             (a3.as_ptr() as usize) % 32);
}
