# Programming-Languages-Mini-Projects
This repository contains multiple PL mini projects demonstrating the programmin skills in:
- rust
- Ocaml and,
- cargo.

### Lexical Analyzer - 
📁 [View Project](./PROJECT%201)

##### This project is about a Lexical Analyzer able yo recognize the tokens from the input into categories like keywords, operators, punctuations, Integer literals, identifiers, Unknown tokens. It is implemented in Ocaml.

##### firstly a Project Dierectory is created with the name "ocaml-lexer" and then a source file is created with the name "Lexical_Analyzer.ml" 

#### Defining the token types
type token =
  | Keyword of string
  | Operator of string
  | Punctuation of char
  | IntLiteral of int
  | Identifier of string
  | Unknown of string

#### Keywords, operators and punctuation list
  let keywords = ["if"; "else"; "while"; "let"; "in"; "then"]
  let operators = ["+"; "-"; "*"; "/"; "=="; "!="; "="]
  let punctuation = ["("; ")"; "{"; "}"; ";"]

#####  Explaination - All the keywords, Operators, punctuation symbols are defined which helps in recognizing them.

#### Classifying the tokens
let classify_token s =
  if List.mem s keywords then Keyword s
  else if List.mem s operators then Operator s
  else if List.mem s punctuation then Punctuation s
  else if Str.string_match (Str.regexp "^-?[0-9]+$") s 0 then IntLiteral (int_of_string s)
  else if Str.string_match (Str.regexp "^[a-zA-Z_][a-zA-Z0-9_]*$") s 0 then Identifier s
  else Unknown s

#####  Explaination - 

#### Spliting the input into tokens and seperates the punctuations
  let split_input input =
  let punct_regex = Str.regexp "\\([();{}]\\)" in
  let spaced_input = Str.global_replace punct_regex " \\1 " input in
  Str.split (Str.regexp "[ \t\n\r]+") spaced_input
  
#### Tokenize the input string and execution of the input
let tokenize input =
  let split_words = split_input input in
  List.map classify_token split_words

let () =
  print_string "Enter input: ";
  let input = read_line () in
  let tokens = tokenize input in
  List.iter (fun t ->
    match t with
    | Keyword k -> Printf.printf "Keyword: %s\n" k
    | Operator o -> Printf.printf "Operator: %s\n" o
    | Punctuation p -> Printf.printf "Punctuation: %s\n" p
    | IntLiteral n -> Printf.printf "IntLiteral: %d\n" n
    | Identifier id -> Printf.printf "Identifier: %s\n" id
    | Unknown u -> Printf.printf "Unknown: %s\n" u
  ) tokens

##### Explaination - After defining, the token are checked and categorized to specific categories they are belonged to. As it checks by spliting the input string into words and checks each one whether it is the word is (checks in an order) keyword, operator, Punctuation, integer Literal, Identifier, Unknown tokens. Then create a funtion to print the data in the required format. And to complile the lexer give the "ocamlc str.cma Lexical_Analyzer.ml -o lexer" command in the ocaml and the run it by giving "./lexer" then it shows "Enter the input", we can give the input we want to and we will get the expected output.

##### As given in the assignment i have tested the two testcase along with two other examples. i have attached the screenshots pdf of the output we got in the ocaml along with this readme file. 

### LL Parser -
📁 [View Project](./PROJECT%202)

This project implements an LL Parser using Rust to process user input and generate the corresponding parsed output. The parser accepts expressions at runtime and analyzes them based on defined parsing logic.

**Input Handling** -
use std::io; --> The standard input/output library is used to read user input from the terminal during program execution.

**Main Function Logic**
fn main() {
    let mut input = String::new();
    println!("Enter the input:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
##### Explanation
The program prompts the user to enter an input string. The input is read from standard input and stored for further parsing.

**Parsing and Processing** -
After reading the input, the program processes it according to LL parsing rules defined within the program logic. The input is analyzed step-by-step and the corresponding parsed result is generated.

**Output Generation** -
Once parsing is completed, the program displays the output related to the given input expression. This helps verify whether the input follows the expected grammar structure.

##### Compilation and Execution : To compile and run the program, use:
cargo build
cargo run
The program will prompt the user to enter input and then display the parsed output.

##### Testing: The program was tested using:
Test cases provided in the assignment
Additional custom input examples

### Cargo -
📁 [View Project](./PROJECT%203)

### 1. How to compile and run - 
#### Rust and Cargo are installed and they are checked whether they are sucessfully installed are or not by using these functions "rustc --version" and "cargo --version".
#### Then a new file is created with name "sql_project" and "src/main.rs" is created in it.
#### The code is then wrote in the "VS code" in main.rs and saved. the dependencies are also saved in cargo.toml file.
#### Then again come to ubuntu, compile and run the code using "cargo build and cargo run". It shows to enter the query, enter it and check whether the query is correct or not.

### 2. Listing the dependencies -
#### --> "sqlparser", version: 0.41.0, this helps to read the SQL query type and breaks into parts which helps the rust to understand the program
#### --> "maplit", version: 1.0.2, it helps us to create Hashmaps easily in rust and also to write tables using key-value pairs. 
#### --> "serde_json", version: 1., this helps to convert rust to JSON and JSON ot rust easily, particularly in this project it helps the query results to print in clear, readable JSON-like format.
### These all the dependencies are all included in "Cargo.toml"

### 3. Sample test case and output
#### The test case are run using "cargo test". 
#### For Example- 
#[test]

fn test_valid_queries() {

    assert!(query_is_correct("SELECT name FROM student WHERE name = 'Alice'"));
    
}
#### This helps in checking the query returns the expected result. 

#[test]

fn test_invalid_queries() {

    assert!(!query_is_correct("SELECT name student WHERE major = 'CS'"));
    
}
#### This checks that Query is incorrect and is rejected.
The output it showed is this:
running 2 tests
test tests::test_invalid_queries ... ok
test tests::test_valid_queries ... ok

test result: ok. 2 passed; 0 failed
This means that both the tests are passed and tells that my SQL is working as expected.


![Screenshot 2025-04-30 183903](https://github.com/user-attachments/assets/a7e42b11-d705-4334-8393-df32d0af9357)
![Screenshot 2025-04-30 183922](https://github.com/user-attachments/assets/77bf4861-f95d-41df-bfa8-134766a60510)
![Screenshot 2025-04-30 184645](https://github.com/user-attachments/assets/9f3ef568-5e6e-4cce-b111-0464ae55358b)

