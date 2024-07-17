/*fn main() {
   println!("Hello, my name is Jesus Martinez, IM going to the moon!");
}

*/

fn main() {
    forloops();
    arrayfor();
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is: {}", result);
    println!("Counter is: {}", counter);
}

fn forloops() {
    let nums = [1, 2, 3, 4, 5];

    // for each loop
    for num in nums.iter() {
        println!("{} ", num);
    }
}
fn arrayfor() {
    let nums = [1, 2, 3, 4, 5];

    // classic loop 
    for idx in 0..nums.len() { 
        // start inclusive ..end exclusive (it's fancy notation for range)
        println!("Element under idx {} := {}", idx, nums[idx]);
    }
}