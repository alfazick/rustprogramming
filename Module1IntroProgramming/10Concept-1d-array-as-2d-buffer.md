# Simulating a 2D Buffer with a 1D Array

In graphics programming, it's common to use a 1D array to represent a 2D image buffer. This technique is efficient and widely used. Let's explore how this works.

## Concept

Imagine a 2D grid of pixels. Each pixel has an (x, y) coordinate:

```
(0,0) (1,0) (2,0) ...
(0,1) (1,1) (2,1) ...
(0,2) (1,2) (2,2) ...
 ...   ...   ...  ...
```

In memory, we store this as a single, long 1D array. The trick is in how we map the 2D coordinates to 1D indices.

## Mapping Formula

The key is this formula:

```
index = y * WIDTH + x
```

Where:
- `index` is the position in the 1D array
- `x` and `y` are the 2D coordinates
- `WIDTH` is the width of the 2D image

## Visual Example

Let's visualize this with a small 4x3 image:

```
2D representation:     1D array representation:
(0,0) (1,0) (2,0) (3,0)  [0]  [1]  [2]  [3]  [4]  [5]  [6]  [7]  [8]  [9] [10] [11]
(0,1) (1,1) (2,1) (3,1)   |    |    |    |    |    |    |    |    |    |    |    |
(0,2) (1,2) (2,2) (3,2)   |____|____|____|____|____|____|____|____|____|____|____|
                         (0,0)(1,0)(2,0)(3,0)(0,1)(1,1)(2,1)(3,1)(0,2)(1,2)(2,2)(3,2)
```

## Code Implementation

Here's how we implement this in our `set_pixel` function:

```rust
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn set_pixel(buffer: &mut [u8], x: usize, y: usize, r: u8, g: u8, b: u8) {
    if x < WIDTH && y < HEIGHT {
        let index = (y * WIDTH + x) * 3;
        buffer[index] = r;
        buffer[index + 1] = g;
        buffer[index + 2] = b;
    }
}
```

Let's break this down:

1. `y * WIDTH` gives us the starting index of the row we're on.
2. Adding `x` moves us to the correct column in that row.
3. We multiply by 3 because each pixel uses 3 bytes (R, G, B).

## Example Calculations

Let's calculate indices for a 64x32 image:

1. Pixel at (0, 0):
   ```
   index = (0 * 64 + 0) * 3 = 0
   ```
   This is the very first pixel, so it starts at index 0.

2. Pixel at (1, 0):
   ```
   index = (0 * 64 + 1) * 3 = 3
   ```
   This is the second pixel in the first row, starting at index 3.

3. Pixel at (0, 1):
   ```
   index = (1 * 64 + 0) * 3 = 192
   ```
   This is the first pixel of the second row.

## Advantages

1. **Memory Efficiency**: One contiguous block of memory is more efficient than a 2D array of pointers.
2. **Cache Friendly**: Sequential memory access is faster due to CPU cache optimization.
3. **Simplicity**: Easier to allocate, deallocate, and pass around as a single chunk of memory.

## Considerations

- Always ensure `x < WIDTH` and `y < HEIGHT` to prevent buffer overruns.
- When working with color values, remember to multiply the index by the number of color components (3 for RGB, 4 for RGBA).

By using this 1D array technique, we can efficiently represent and manipulate 2D image data, which is crucial for performance in graphics programming.
