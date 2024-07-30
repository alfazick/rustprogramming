# Complete LLM Prompt Engineering Example with Enums and RNG in Rust

LLM prompt engineering using enums and incorporating randomness:

```rust
use rand::Rng;

// Enum to represent different prompt strategies
#[derive(Clone)]
enum PromptStrategy {
    Direct,
    ChainOfThought,
    FewShot(Vec<String>),
    SelfConsistency(u32), // Number of reasoning paths
}

// Enum to represent different question types
enum QuestionType {
    Factual,
    Analytical,
    Creative,
    Mathematical,
}

// Struct to represent a question
struct Question {
    text: String,
    question_type: QuestionType,
}

// Function to generate a prompt based on the strategy and question
fn generate_prompt(question: &Question, strategy: &PromptStrategy) -> String {
    match strategy {
        PromptStrategy::Direct => {
            format!("Please answer the following question: {}", question.text)
        }
        PromptStrategy::ChainOfThought => {
            format!(
                "Let's approach this step-by-step:\n1) Understand the question: {}\n2) Think about relevant information\n3) Reason through the steps\n4) Formulate the answer\nNow, let's go through this process:",
                question.text
            )
        }
        PromptStrategy::FewShot(examples) => {
            let mut prompt = String::from("Here are a few examples:\n");
            for (i, example) in examples.iter().enumerate() {
                prompt.push_str(&format!("Example {}: {}\n", i + 1, example));
            }
            prompt.push_str(&format!("Now, please answer: {}", question.text));
            prompt
        }
        PromptStrategy::SelfConsistency(paths) => {
            format!(
                "Please provide {} different reasoning paths to answer the following question, then synthesize them into a final answer: {}",
                paths, question.text
            )
        }
    }
}

// Function to select the best prompt strategy for a given question type
fn select_prompt_strategy(question_type: &QuestionType) -> PromptStrategy {
    match question_type {
        QuestionType::Factual => PromptStrategy::Direct,
        QuestionType::Analytical => PromptStrategy::ChainOfThought,
        QuestionType::Creative => PromptStrategy::FewShot(vec![
            "Creative Example 1: [...]".to_string(),
            "Creative Example 2: [...]".to_string(),
        ]),
        QuestionType::Mathematical => PromptStrategy::SelfConsistency(3),
    }
}

// Simulated LLM function with randomness
fn simulate_llm_response(prompt: &str) -> String {
    let mut rng = rand::thread_rng();
    let response_quality: f64 = rng.gen(); // Random float between 0 and 1

    let base_response = format!("LLM Response to: {}", prompt);
    
    if response_quality < 0.2 {
        format!("{} [Low quality response]", base_response)
    } else if response_quality < 0.8 {
        format!("{} [Average quality response]", base_response)
    } else {
        format!("{} [High quality response]", base_response)
    }
}

// Ensemble function with random strategy selection
fn ensemble_prompt_engineering(question: &Question) -> String {
    let mut rng = rand::thread_rng();
    let strategies = vec![
        PromptStrategy::Direct,
        PromptStrategy::ChainOfThought,
        PromptStrategy::FewShot(vec!["Example: [...]".to_string()]),
        PromptStrategy::SelfConsistency(2),
    ];

    let mut responses = Vec::new();

    for _ in 0..3 { // Randomly select 3 strategies
        let strategy = strategies[rng.gen_range(0..strategies.len())].clone();
        let prompt = generate_prompt(question, &strategy);
        let response = simulate_llm_response(&prompt);
        responses.push(response);
    }

    // Combine responses, simulating a simple voting mechanism
    let combined = responses.join("\n");
    if combined.contains("High quality response") {
        format!("Ensemble High Quality: {}", combined)
    } else if combined.contains("Low quality response") {
        format!("Ensemble Low Quality: {}", combined)
    } else {
        format!("Ensemble Average Quality: {}", combined)
    }
}

fn main() {
    let questions = vec![
        Question {
            text: "What is the capital of France?".to_string(),
            question_type: QuestionType::Factual,
        },
        Question {
            text: "Analyze the causes of the Industrial Revolution.".to_string(),
            question_type: QuestionType::Analytical,
        },
        Question {
            text: "Write a short story about a time-traveling robot.".to_string(),
            question_type: QuestionType::Creative,
        },
        Question {
            text: "Solve the equation: 2x + 5 = 13".to_string(),
            question_type: QuestionType::Mathematical,
        },
    ];

    for question in questions {
        println!("Question: {}", question.text);
        
        // Using the best strategy for the question type
        let best_strategy = select_prompt_strategy(&question.question_type);
        let best_prompt = generate_prompt(&question, &best_strategy);
        println!("Best Strategy Prompt:\n{}", best_prompt);
        println!("Best Strategy Response: {}", simulate_llm_response(&best_prompt));

        // Using the ensemble approach
        println!("Ensemble Response:\n{}", ensemble_prompt_engineering(&question));
        println!("---");
    }
}
```

This complete example demonstrates:

1. Use of enums (`PromptStrategy` and `QuestionType`) to represent different prompt strategies and question types.
2. A `Question` struct to encapsulate question data.
3. Functions for generating prompts and selecting strategies based on question types.
4. A simulated LLM response function that uses `rand::Rng` to introduce variability in response quality.
5. An ensemble prompt engineering approach that randomly selects strategies and combines their results.
6. Use of `rand::Rng` in both the `simulate_llm_response` and `ensemble_prompt_engineering` functions to add realistic randomness.

To run this code, you'll need to add the `rand` crate to your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8.5"
```

This example provides a comprehensive demonstration of how enums can be used in conjunction with randomness to create a flexible and realistic simulation of LLM prompt engineering strategies.
