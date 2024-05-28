use std::collections::HashMap;

fn number_to_words(n: &str) -> String {
    // Maps for single digits, teens, and tens
    let units: HashMap<u32, &str> = [
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ]
    .iter()
    .cloned()
    .collect();

    let teens: HashMap<u32, &str> = [
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
    ]
    .iter()
    .cloned()
    .collect();

    let tens: HashMap<u32, &str> = [
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
    ]
    .iter()
    .cloned()
    .collect();

    // Split integer and decimal parts
    let parts: Vec<&str> = n.split('.').collect();
    let integer_part = parts[0].replace(",", "");
    let decimal_part = if parts.len() > 1 { parts[1] } else { "" };

    // Convert integer part
    let integer_words = if integer_part.is_empty() {
        String::new()
    } else {
        integer_to_words(&integer_part, &units, &teens, &tens)
    };

    // Convert decimal part
    let decimal_words = if decimal_part.is_empty() {
        String::new()
    } else {
        let decimal_digits: Vec<String> = decimal_part
            .chars()
            .map(|c| units[&(c.to_digit(10).unwrap())].to_string())
            .collect();
        format!("point {}", decimal_digits.join(" "))
    };

    // Combine integer and decimal parts
    let result = if decimal_words.is_empty() {
        integer_words
    } else {
        format!("{} {}", integer_words, decimal_words)
    };

    result.trim().to_string()
}

fn integer_to_words(
    n: &str,
    units: &HashMap<u32, &str>,
    teens: &HashMap<u32, &str>,
    tens: &HashMap<u32, &str>,
) -> String {
    let num: u64 = n.parse().unwrap();

    if num == 0 {
        return "zero".to_string();
    }

    let mut words = String::new();
    let chunks = split_number_into_chunks(num);

    for (i, chunk) in chunks.iter().enumerate() {
        if *chunk == 0 {
            continue;
        }

        let chunk_words = chunk_to_words(*chunk, units, teens, tens);
        if !words.is_empty() {
            words = format!("{}, {}", chunk_words, words);
        } else {
            words = chunk_words;
        }

        if i > 0 {
            let scale = match i {
                1 => "thousand",
                2 => "million",
                3 => "billion",
                _ => "",
            };
            words = format!("{} {}", scale, words);
        }
    }

    words
}

fn chunk_to_words(
    n: u32,
    units: &HashMap<u32, &str>,
    teens: &HashMap<u32, &str>,
    tens: &HashMap<u32, &str>,
) -> String {
    let mut words = String::new();

    let hundreds = n / 100;
    let remainder = n % 100;

    if hundreds > 0 {
        words = format!("{} hundred", units[&hundreds]);
    }

    if remainder > 0 {
        if !words.is_empty() {
            words = format!("{} and ", words);
        }

        if remainder < 10 {
            words = format!("{}{}", words, units[&remainder]);
        } else if remainder < 20 {
            words = format!("{}{}", words, teens[&remainder]);
        } else {
            let tens_value = (remainder / 10) * 10;
            let units_value = remainder % 10;

            words = format!("{}{}", words, tens[&tens_value]);

            if units_value > 0 {
                words = format!("{}-{}", words, units[&units_value]);
            }
        }
    }

    words
}

fn split_number_into_chunks(num: u64) -> Vec<u32> {
    let mut chunks = Vec::new();
    let mut n = num;

    while n > 0 {
        chunks.push((n % 1000) as u32);
        n /= 1000;
    }

    chunks
}

fn main() {
    let test_numbers = vec![
        "1",
        "1.0",
        "1.4564",
        "0",
        "0.1",
        "1.2",
        "12",
        "123",
        "1234",
        "9,239.12",
        "123,450",
        "1234562",
        "12345678",
        "1,234,567,899",
    ];

    for number in test_numbers {
        println!("{} -> {}", number, number_to_words(number));
    }
}
