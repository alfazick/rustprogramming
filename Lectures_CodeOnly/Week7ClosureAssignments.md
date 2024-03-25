Assignment # 1
Easy: Implementing a Functional Array Transformer
Objective: Create a function that applies a transformation to each element of an array, using a closure as the transformation logic.

Task Details:

Write a function named transform_array that accepts:
A slice of i32 values.
A closure that takes an i32 value and returns an i32.
The function should return a new Vec<i32> where each element is the result of applying the closure to the corresponding element in the input slice.
Demonstrate using transform_array with at least two different closures: one that squares each element and another that doubles each element.


Solution:

fn transform_array<F>(input: &[i32], transform: F) -> Vec<i32>
where 
    F: Fn(i32) ->i32,
{
    input.iter().map(|&x| transform(x)).collect()
}
fn main(){

    let double = |x| x * x;


    let input = [1,2,3,4];
    let squared = transform_array(&input,double);
    let multiple = transform_array(&input,|x| x*2);

    println!("{:?}",squared);
    println!("{:?}",multiple);

}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_transform() {
        let input = [1, 2, 3, 4];
        let expected = [1, 4, 9, 16];
        assert_eq!(transform_array(&input, |x| x * x), expected);
    }

    #[test]
    fn test_double_transform() {
        let input = [1, 2, 3, 4];
        let expected = [2, 4, 6, 8];
        assert_eq!(transform_array(&input, |x| x * 2), expected);
    }
}

Assignment #2
 Dynamic Sorting Based on a Single, Selectable Criterion
Objective: Implement a flexible sorting mechanism for a list of Employee structs that allows sorting based on a single, dynamically selected attribute (e.g., by name or by age) using closures.

#[derive(Debug, PartialEq, Eq, Clone)]
struct Employee {
    name: String,
    age: u32,
}

struct EmployeeDatabase {
    employees: Vec<Employee>,
    sort_by: Box<dyn Fn(&Employee) -> u32>, // Sorting based on a u32 value, simplifying to age for example
}

impl EmployeeDatabase {
    fn new(employees: Vec<Employee>, sort_by: Box<dyn Fn(&Employee) -> u32>) -> Self {
        EmployeeDatabase { employees, sort_by }
    }

    fn sort(&mut self) {
        self.employees.sort_by_key(|emp| (self.sort_by)(emp));
    }

    fn update_sort_criterion(&mut self, new_sort_by: Box<dyn Fn(&Employee) -> u32>) {
        self.sort_by = new_sort_by;
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_age() {
        let mut db = EmployeeDatabase::new(
            vec![
                Employee { name: "Charlie".to_string(), age: 30 },
                Employee { name: "Alice".to_string(), age: 28 },
                Employee { name: "Bob".to_string(), age: 25 },
            ],
            Box::new(|e| e.age),
        );

        db.sort();

        let expected = vec![
            Employee { name: "Bob".to_string(), age: 25 },
            Employee { name: "Alice".to_string(), age: 28 },
            Employee { name: "Charlie".to_string(), age: 30 },
        ];

        assert_eq!(db.employees, expected);
    }

    #[test]
    fn test_dynamic_sort_update() {
        let mut db = EmployeeDatabase::new(
            vec![
                Employee { name: "Charlie".to_string(), age: 30 },
                Employee { name: "Bob".to_string(), age: 25 },
                Employee { name: "Alice".to_string(), age: 28 },
            ],
            Box::new(|e| e.age),
        );

        // Initially sorted by age
        db.sort();
        assert_eq!(db.employees[0].name, "Bob");

        // Update criterion to sort by the length of the name
        db.update_sort_criterion(Box::new(|e| e.name.len() as u32));
        db.sort();
        assert_eq!(db.employees[0].name, "Bob"); // Assuming sorting by name length, Bob should still be first
    }
}
