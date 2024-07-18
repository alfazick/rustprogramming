// Jesus Martinez July 17,2024
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    return false;
}
fn main() {
    let arr = [45, 15, 20, 63, 30, 42, 5, 10, 35, 2];
    let mut num = 0;
    loop {
        if num < arr.len() {
            if is_even(arr[num]) {
                println!("Number {}, is even", arr[num]);
            } else {
                println!("Number {}, is odd", arr[num]);
            }

            if arr[num] % 3 == 0 && arr[num] % 5 == 0 {
                println!("FizzBuzz\n");
            } else if arr[num] % 3 == 0 {
                println!("Fizz\n");
            } else if arr[num] % 5 == 0 {
                println!("Buzz\n");
            }
        } else {
            break;
        }
        num += 1;
    }
    let mut sum = 0;
    let mut largest = arr[0];
    while num != 0 {
        num -= 1;
        sum = sum + arr[num];
        if largest < arr[num] {
            largest = arr[num];
        }
    }
    println!("The sum of the array is {}\n", sum);
    println!("The largest of the array is {}\n", largest);
}
