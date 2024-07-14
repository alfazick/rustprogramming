# Rust Graphics: Theory and Implementation with Plain Arrays

## Introduction to Frame Buffers

A frame buffer is a portion of memory that contains a bitmap representing the pixels on a display device. In computer graphics, manipulating this buffer allows us to control what appears on the screen. Each pixel in the frame buffer typically consists of color information, usually in RGB (Red, Green, Blue) format.

## Simulating a 2D Buffer with a 1D Array

While we conceptualize our display as a 2D grid of pixels, in memory, we typically use a 1D array to represent this data. This approach is memory-efficient and cache-friendly.

### Mapping 2D to 1D

We use the following formula to map 2D coordinates to a 1D index:

```
index = (y * WIDTH + x) * 3
```

Where:
- `index` is the position in the 1D array
- `x` and `y` are the 2D coordinates
- `WIDTH` is the width of the 2D image
- We multiply by 3 because each pixel uses 3 bytes (R, G, B)

### Visual Example

Let's visualize this with a small 4x3 image:

```
2D representation:     1D array representation:
(0,0) (1,0) (2,0) (3,0)  [0]  [1]  [2]  [3]  [4]  [5]  [6]  [7]  [8]  [9] [10] [11]
(0,1) (1,1) (2,1) (3,1)   |    |    |    |    |    |    |    |    |    |    |    |
(0,2) (1,2) (2,2) (3,2)   |____|____|____|____|____|____|____|____|____|____|____|
                         (0,0)(1,0)(2,0)(3,0)(0,1)(1,1)(2,1)(3,1)(0,2)(1,2)(2,2)(3,2)
```

Each cell in the 1D array actually represents three consecutive elements for R, G, and B values.

## Implementation in Rust using Plain Arrays

Now, let's implement this concept in Rust using plain arrays instead of vectors. This approach can be more efficient when the buffer size is known at compile time.

### Constants and Buffer Setup

```rust
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const BUFFER_SIZE: usize = WIDTH * HEIGHT * 3; // 3 bytes per pixel (RGB)

// In main function:
let mut frame_buffer = [0u8; BUFFER_SIZE];
```

Here, we're declaring `frame_buffer` as a fixed-size array of `u8` with `BUFFER_SIZE` elements, all initialized to zero.

### Pixel Manipulation

```rust
fn set_pixel(buffer: &mut [u8; BUFFER_SIZE], x: usize, y: usize, r: u8, g: u8, b: u8) {
    if x < WIDTH && y < HEIGHT {
        let index = (y * WIDTH + x) * 3;
        buffer[index] = r;
        buffer[index + 1] = g;
        buffer[index + 2] = b;
    }
}
```

This function sets the color of a pixel at coordinates (x, y). It first checks if the coordinates are within bounds, then calculates the index in the 1D array and sets the R, G, and B values.

### Buffer Clearing

```rust
fn clear_buffer(buffer: &mut [u8; BUFFER_SIZE], r: u8, g: u8, b: u8) {
    for i in (0..BUFFER_SIZE).step_by(3) {
        buffer[i] = r;
        buffer[i + 1] = g;
        buffer[i + 2] = b;
    }
}
```

This function fills the entire buffer with a specified color, effectively clearing the screen.

### Drawing the Number One

```rust
fn draw_one(buffer: &mut [u8; BUFFER_SIZE]) {
    clear_buffer(buffer, 0, 0, 0);  // Clear to black

    let column = WIDTH / 2;
    let start_row = HEIGHT / 4;
    let end_row = 3 * HEIGHT / 4;

    // Draw vertical line
    for y in start_row..end_row {
        set_pixel(buffer, column, y, 255, 255, 255);
    }

    // Draw base of '1'
    for x in (column - 2)..=(column + 2) {
        set_pixel(buffer, x, end_row, 255, 255, 255);
    }

    // Draw top of '1'
    for x in (column - 2)..column {
        set_pixel(buffer, x, start_row, 255, 255, 255);
    }
}
```

This function demonstrates how to draw a simple shape (the number "1") by manipulating individual pixels.

### Buffer Display

```rust
fn display_buffer(buffer: &[u8; BUFFER_SIZE]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = (y * WIDTH + x) * 3;
            let avg = (buffer[index] as u32 + buffer[index + 1] as u32 + buffer[index + 2] as u32) / 3;
            print!("{}", if avg > 128 { "█" } else { "·" });
        }
        println!();
    }
}
```

This function creates a simple ASCII representation of the buffer, using different characters based on the brightness of each pixel.

### Complete Code

Here's the complete implementation:

```rust
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const BUFFER_SIZE: usize = WIDTH * HEIGHT * 3;

fn set_pixel(buffer: &mut [u8; BUFFER_SIZE], x: usize, y: usize, r: u8, g: u8, b: u8) {
    if x < WIDTH && y < HEIGHT {
        let index = (y * WIDTH + x) * 3;
        buffer[index] = r;
        buffer[index + 1] = g;
        buffer[index + 2] = b;
    }
}

fn clear_buffer(buffer: &mut [u8; BUFFER_SIZE], r: u8, g: u8, b: u8) {
    for i in (0..BUFFER_SIZE).step_by(3) {
        buffer[i] = r;
        buffer[i + 1] = g;
        buffer[i + 2] = b;
    }
}

fn draw_one(buffer: &mut [u8; BUFFER_SIZE]) {
    clear_buffer(buffer, 0, 0, 0);  // Clear to black

    let column = WIDTH / 2;
    let start_row = HEIGHT / 4;
    let end_row = 3 * HEIGHT / 4;

    // Draw vertical line
    for y in start_row..end_row {
        set_pixel(buffer, column, y, 255, 255, 255);
    }

    // Draw base of '1'
    for x in (column - 2)..=(column + 2) {
        set_pixel(buffer, x, end_row, 255, 255, 255);
    }

    // Draw top of '1'
    for x in (column - 2)..column {
        set_pixel(buffer, x, start_row, 255, 255, 255);
    }
}

fn display_buffer(buffer: &[u8; BUFFER_SIZE]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = (y * WIDTH + x) * 3;
            let avg = (buffer[index] as u32 + buffer[index + 1] as u32 + buffer[index + 2] as u32) / 3;
            print!("{}", if avg > 128 { "█" } else { "·" });
        }
        println!();
    }
}

fn main() {
    let mut frame_buffer = [0u8; BUFFER_SIZE];
    
    // Draw and display number one
    draw_one(&mut frame_buffer);
    println!("Number One:");
    display_buffer(&frame_buffer);
}
```

## Advantages of Using Arrays

1. **Fixed Size**: Arrays have a fixed size known at compile time, which can lead to better performance optimizations by the compiler.
2. **Stack Allocation**: For smaller buffers, arrays are typically allocated on the stack, which is faster than heap allocation used by `Vec`.
3. **No Dynamic Resizing**: Arrays cannot be resized, which eliminates the overhead of potential reallocations.
4. **Compile-Time Size Checking**: The compiler can catch buffer size mismatches at compile time, potentially preventing runtime errors.

## Considerations

- Arrays are less flexible if you need to change the buffer size at runtime.
- For very large buffers, stack overflow can occur if the array is too large for the stack.
- In graphics programming, where buffer sizes are often known in advance and performance is crucial, using arrays can be a good choice.

This implementation demonstrates how to use a 1D array to simulate a 2D graphics buffer in Rust, providing a foundation for more complex graphics programming tasks.
