# Number to Words Converter - written by ChatGPT-4o

Experiment inspired by this [blogpost from Github](https://github.blog/2024-04-09-4-ways-github-engineers-use-github-copilot/#4-exploring-and-learning)

It was my first time with Rust (same as the author of the original experiment from GitHub Team).
But instead of comments in VSCode with Github Copilot I used ChatGPT-4o chat window with below prompt:

## Initial prompt to ChatGPT-4o

Generate a function in Rust converting any numerical input into its written English equivalent (a string). It should handle teen numbers, naming conventions for tens, placement of “and” in the output, and dots `.` and commas `,` in the input number.
examples:

```
1 -> one
1.0 -> one point zero
1.4564 -> one point four five six four
0 -> zero
0.1 -> zero point one
1.2 -> one point two
12 -> twelve
123 -> one hundred and twenty-three
1234 -> one thousand, two hundred and thirty-four
9,239.12 -> nine thousand, two hundred and thirty-nine point one two
123,450 -> one hundred and twenty-three thousand, four hundred and fifty
1234562 -> one million, two hundred and thirty-four thousand, five hundred and sixty two
12345678 -> twelve million, three hundred and forty-five thousand, six hundred and seventy-eight
1,234,567,899 -> one billion, two hundred and thirty-four million, five hundred and sixty-seven thousand, eight hundred and ninety-nine
```

> Examples were generated with help of Google Code Assist in VSCode.
> It generated wrong strings for long numbers so I fixed it manually a little bit ;)

## Errors and fixes made by GPT-4o and Replit AI

- Initial results were close, but not fully correct.
- In code I've added comments with description of fixes made by GPT-4o (`main.rs`) and Replit AI (in a separate file).
- In commit history you will find all the changes after fixes.


## "Project" Description

> Whole below text was generated in 90% by Replit AI (with my small changes).

This Rust project converts numbers into their corresponding English words representation. It handles both integers and decimal numbers, including large numbers with thousands and millions, and properly formats them with commas as needed.

## Features

- Converts integers and decimal numbers to their English words representation
- Handles commas and floating numbers in input numbers appropriately
- Outputs numbers in the correct grammatical order
- Supports large numbers up to billions

## Examples

Here are a few examples of the expected output:

```bash
1 -> one
1.0 -> one point zero
1.4564 -> one point four five six four
0 -> zero
0.1 -> zero point one
1.2 -> one point two
12 -> twelve
123 -> one hundred and twenty-three
1234 -> one thousand, two hundred and thirty-four
9,239.12 -> nine thousand, two hundred and thirty-nine point one two
123,450 -> one hundred and twenty-three thousand, four hundred and fifty
1234562 -> one million, two hundred and thirty-four thousand, five hundred and sixty-two
12345678 -> twelve million, three hundred and forty-five thousand, six hundred and seventy-eight
1,234,567,899 -> one billion, two hundred and thirty-four million, five hundred and sixty-seven thousand, eight hundred and ninety-nine
```


## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:

    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. Build the project:

    ```sh
    cargo build
    ```

3. Run the project:

    ```sh
    cargo run
    ```

### Running on Replit:
Alternatively you can just use [this repl](https://replit.com/@LucasMat/Number-to-English-Convertion) and run the code directly in [Replit.com](https://replit.com/) :)


### Running Tests

There are currently no tests provided. However, you can manually verify the output by using the `main` function, as shown in the `src/main.rs` file.

## Project Structure

- `src/main.rs`: The main file containing the logic for converting numbers to words.
- `src/main-ReplitAI-fix.rs`: just to show how Replit AI tried to fix the issue (and failed after 2 tries)

## Code Overview

The project uses the following functions and concepts:

- `number_to_words`: Splits the number into integer and decimal parts, and converts each part into words.
- `integer_to_words`: Converts the integer part of the number into words, handling large numbers by breaking them into chunks (thousands, millions, billions).
- `chunk_to_words`: Converts each chunk of the number (up to three digits) into words.
- `split_number_into_chunks`: Splits the number into chunks of three digits for easier processing.

The program makes extensive use of `HashMap` to store the English word equivalents for single digits, teens, and tens.

## Contributions

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or suggestions.

## License

This project is licensed under the MIT License.

## Acknowledgements

"Project" inspired by this [blogpost from Github](https://github.blog/2024-04-09-4-ways-github-engineers-use-github-copilot/#4-exploring-and-learning)
