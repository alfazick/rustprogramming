# ETL Pipeline in Rust: Line-by-Line Explanation

## Data Structure

```rust
struct DataRecord {
    id: u32,
    value: String,
}
```
This defines a struct `DataRecord` to represent our data. It has two fields:
- `id`: An unsigned 32-bit integer to uniquely identify each record.
- `value`: A String to hold the actual data content.

## ETL Operations

### Extract
```rust
fn extract(_source: &str) -> Vec<DataRecord> {
    vec![
        DataRecord { id: 1, value: String::from("data1") },
        DataRecord { id: 2, value: String::from("data2") },
    ]
}
```
- This function simulates data extraction.
- It takes a `&str` parameter `_source` (unused, hence the underscore).
- Returns a `Vec<DataRecord>` with sample data.

### Transform (Uppercase)
```rust
fn transform_uppercase(record: DataRecord) -> DataRecord {
    DataRecord { id: record.id, value: record.value.to_uppercase() }
}
```
- Takes a `DataRecord` and returns a new `DataRecord`.
- Keeps the `id` the same but converts the `value` to uppercase.

### Transform (Add Prefix)
```rust
fn transform_add_prefix(record: DataRecord) -> DataRecord {
    DataRecord { id: record.id, value: format!("PREFIX_{}", record.value) }
}
```
- Takes a `DataRecord` and returns a new `DataRecord`.
- Keeps the `id` the same but adds "PREFIX_" to the beginning of the `value`.

### Load
```rust
fn load(records: Vec<DataRecord>) {
    for record in records {
        println!("Loaded: id={}, value={}", record.id, record.value);
    }
}
```
- Takes a `Vec<DataRecord>` and prints each record.
- Simulates a data loading process.

## ETL Pipeline Structure

```rust
struct EtlPipeline {
    extract: fn(&str) -> Vec<DataRecord>,
    transform1: fn(DataRecord) -> DataRecord,
    transform2: fn(DataRecord) -> DataRecord,
    load: fn(Vec<DataRecord>),
}
```
This struct represents the ETL pipeline:
- `extract`: Function pointer for the extraction step.
- `transform1` and `transform2`: Function pointers for two transformation steps.
- `load`: Function pointer for the loading step.

## ETL Pipeline Implementation

```rust
impl EtlPipeline {
    fn new(
        extract: fn(&str) -> Vec<DataRecord>,
        transform1: fn(DataRecord) -> DataRecord,
        transform2: fn(DataRecord) -> DataRecord,
        load: fn(Vec<DataRecord>)
    ) -> Self {
        EtlPipeline { extract, transform1, transform2, load }
    }
```
- `new` is a constructor for `EtlPipeline`.
- Takes function pointers for each ETL step.
- Returns a new `EtlPipeline` instance.

```rust
    fn run(&self, source: &str) {
        let data = (self.extract)(source);
        let transformed_data: Vec<_> = data.into_iter()
            .map(self.transform1)
            .map(self.transform2)
            .collect();
        (self.load)(transformed_data);
    }
}
```
- `run` executes the ETL pipeline:
  1. Calls the `extract` function.
  2. Applies `transform1` and `transform2` in sequence.
  3. Calls the `load` function with the transformed data.

## Main Function

```rust
fn main() {
    let etl_pipeline = EtlPipeline::new(extract, transform_uppercase, transform_add_prefix, load);
    etl_pipeline.run("dummy_source");
}
```
- Creates a new `EtlPipeline` instance with our defined functions.
- Runs the pipeline with a dummy source string.

## Complete Code

```rust
struct DataRecord {
    id: u32,
    value: String,
}

fn extract(_source: &str) -> Vec<DataRecord> {
    vec![
        DataRecord { id: 1, value: String::from("data1") },
        DataRecord { id: 2, value: String::from("data2") },
    ]
}

fn transform_uppercase(record: DataRecord) -> DataRecord {
    DataRecord { id: record.id, value: record.value.to_uppercase() }
}

fn transform_add_prefix(record: DataRecord) -> DataRecord {
    DataRecord { id: record.id, value: format!("PREFIX_{}", record.value) }
}

fn load(records: Vec<DataRecord>) {
    for record in records {
        println!("Loaded: id={}, value={}", record.id, record.value);
    }
}

struct EtlPipeline {
    extract: fn(&str) -> Vec<DataRecord>,
    transform1: fn(DataRecord) -> DataRecord,
    transform2: fn(DataRecord) -> DataRecord,
    load: fn(Vec<DataRecord>),
}

impl EtlPipeline {
    fn new(
        extract: fn(&str) -> Vec<DataRecord>,
        transform1: fn(DataRecord) -> DataRecord,
        transform2: fn(DataRecord) -> DataRecord,
        load: fn(Vec<DataRecord>)
    ) -> Self {
        EtlPipeline { extract, transform1, transform2, load }
    }

    fn run(&self, source: &str) {
        let data = (self.extract)(source);
        let transformed_data: Vec<_> = data.into_iter()
            .map(self.transform1)
            .map(self.transform2)
            .collect();
        (self.load)(transformed_data);
    }
}

fn main() {
    let etl_pipeline = EtlPipeline::new(extract, transform_uppercase, transform_add_prefix, load);
    etl_pipeline.run("dummy_source");
}
```

This code demonstrates a flexible ETL pipeline using function composition without closures, allowing for easy modification and extension of data processing logic.

