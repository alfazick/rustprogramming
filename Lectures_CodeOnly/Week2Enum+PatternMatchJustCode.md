


fn simple_enum(){
    // Define an enum for payment methods
    #[derive(Debug)]
    enum PaymentMethod {
        Cash,
        CreditCard,
    }

    let cash = PaymentMethod::Cash;
    let cc = PaymentMethod::CreditCard;

    println!("{:?}",cash);
    println!("{:?}",cc);
}


fn enum_as_struct_field(){

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Genre {
        Fiction,
        NonFiction,
        ScienceFiction,
    }
    
    struct Book {
        title: String,
        genre: Genre,
        review_score: u8,
    }
    
    
    let my_book = Book {
        title: String::from("Rust Programming"),
        genre: Genre::NonFiction,
        review_score: 5,
    };

    println!("{} is a {:?} book with a score of {}", my_book.title, my_book.genre, my_book.review_score);

    
}


fn classic_usage_enum(){

    #[derive(PartialEq)]
    #[derive(Debug)]
    #[allow(dead_code)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    
    let light = TrafficLight::Green;

    if light == TrafficLight::Green {
        println!("Go!");
    } else if light == TrafficLight::Yellow {
        println!("Prepare to stop.");
    } else {
        println!("Stop.");
    }
    
    
}

fn intro_pattern_match(){


    fn simple_match(number: i32) {
        match number {
            1 => println!("It is one!"),
            2 => println!("It is two!"),
            _ => println!("It is something else."),
        }
    }
    
    
    simple_match(1);
    simple_match(3);

}

fn enum_pattern_are_best_friends(){

    enum WeatherCondition {
        Sunny,
        Rainy,
        Windy,
    }
    
    fn weather_message(condition: WeatherCondition) {
        match condition {
            WeatherCondition::Sunny => println!("It's a bright sunny day!"),
            WeatherCondition::Rainy => println!("It's raining. Don't forget your umbrella!"),
            WeatherCondition::Windy => println!("It's windy. Wear something warm!"),
        }
    }
    
    
    weather_message(WeatherCondition::Sunny);
    weather_message(WeatherCondition::Rainy);
    
    

}
// make sure to add to cargo.toml
//[dependencies]
//chrono = "0.4"


fn enum_can_hold_data(){
    use std::time::{SystemTime, Duration};
    use chrono::{DateTime, Utc, TimeZone};

    enum BookStatus {
        Available,
        CheckedOut(SystemTime), // Due date
        InRepair(String), // Notes on the repair
    }
    
    struct Book {
        title: String,
        status: BookStatus,
    }
    

    fn display_book_status(book: &Book) {
        match &book.status {
            BookStatus::Available => println!("{} is available for borrowing.", book.title),
            BookStatus::CheckedOut(due_date) => {
                // Convert SystemTime to DateTime<Utc>
                let datetime: DateTime<Utc> = due_date.clone().into();
                // Format the DateTime<Utc> to a string
                let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                println!("{} is checked out. Due date: {}", book.title, formatted_date);
            },
            BookStatus::InRepair(notes) => println!("{} is in repair. Notes: {}", book.title, notes),
        }
    }

    
    let due_date = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 14); // 14 days from now

    let book = Book {
        title: String::from("Rust Programming"),
        status: BookStatus::CheckedOut(due_date),
    };

    display_book_status(&book);
    

    }


fn reading_from_console(){
    use std::io;


    println!("What's your favorite city?");

    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read line");

    println!("Your favorite city is: {}", city.trim());

}



fn main(){
    println!("Enum+Pattern Matching");
    simple_enum();
    enum_as_struct_field();
    classic_usage_enum();
    intro_pattern_match();
    enum_pattern_are_best_friends();
    enum_can_hold_data();
    reading_from_console();
}






//HTML page generator. Enjoy coding.

enum ContentType {
    Heading(String),
    Paragraph(String),
    Link(String, String), // URL, Text
}

struct HtmlElement {
    content_type: ContentType,
}

impl HtmlElement {
    fn new(content_type: ContentType) -> Self {
        HtmlElement { content_type }
    }

    fn render(&self) -> String {
        match &self.content_type {
            ContentType::Heading(text) => format!("<h1>{}</h1>", text),
            ContentType::Paragraph(text) => format!("<p>{}</p>", text),
            ContentType::Link(href, text) => format!("<a href='{}'>{}</a>", href, text),
        }
    }
}

struct HtmlPage {
    title: String,
    elements: Vec<HtmlElement>,
}

impl HtmlPage {
    fn new(title: String) -> Self {
        HtmlPage {
            title,
            elements: Vec::new(),
        }
    }

    fn add_element(&mut self, element: HtmlElement) {
        self.elements.push(element);
    }

    fn generate(&self) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", self.title));
        html.push_str("</head>\n<body>\n");

        for element in &self.elements {
            html.push_str(&element.render());
            html.push('\n');
        }

        html.push_str("</body>\n</html>");
        html
    }
}


use std::fs::File;
use std::io::{self, Write, stdin, stdout};

fn read_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut page = HtmlPage::new(read_user_input("Enter the title of your page:"));
    
    loop {
        println!("\nWhat would you like to add to your page?");
        println!("1: Heading");
        println!("2: Paragraph");
        println!("3: Link");
        println!("4: Generate Page and Exit");

        let choice = read_user_input("Choose an option (1-4):");

        match choice.as_str() {
            "1" => {
                let heading = read_user_input("Enter heading text:");
                page.add_element(HtmlElement::new(ContentType::Heading(heading)));
            },
            "2" => {
                let paragraph = read_user_input("Enter paragraph text:");
                page.add_element(HtmlElement::new(ContentType::Paragraph(paragraph)));
            },
            "3" => {
                let text = read_user_input("Enter link text:");
                let url = read_user_input("Enter link URL:");
                page.add_element(HtmlElement::new(ContentType::Link(url, text)));
            },
            "4" => break,
            _ => println!("Invalid choice, please enter a number between 1 and 4."),
        }
    }

    let html = page.generate();
    let mut file = File::create("interactive_output.html").expect("Could not create file");
    writeln!(file, "{}", html).expect("Could not write to file");

    println!("HTML page generated successfully.");
}

