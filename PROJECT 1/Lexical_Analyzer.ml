  (* Defining the token types *)
  type token =
  | Keyword of string
  | Operator of string
  | Punctuation of string
  | IntLiteral of int
  | Identifier of string
  | Unknown of string
  
  (* list of keywords, operators, and punctuation *)
  let keywords = ["if"; "else"; "while"; "let"; "in"; "then"]
  let operators = ["+"; "-"; "*"; "/"; "=="; "!="; "="]
  let punctuation = ["("; ")"; "{"; "}"; ";"]
  
  (* Classifying the tokens *)
  let classify_token s =
  if List.mem s keywords then Keyword s
  else if List.mem s operators then Operator s
  else if List.mem s punctuation then Punctuation s
  else if Str.string_match (Str.regexp "^-?[0-9]+$") s 0 then IntLiteral (int_of_string s)
  else if Str.string_match (Str.regexp "^[a-zA-Z_][a-zA-Z0-9_]*$") s 0 then Identifier s
  else Unknown s
  
  (*Split input into tokens and separating punctuation *)
  let split_input input =
  let punct_regex = Str.regexp "\\([();{}]\\)" in
  let spaced_input = Str.global_replace punct_regex " \\1 " input in
  Str.split (Str.regexp "[ \t\n\r]+") spaced_input
  
  (* Tokenize an input string *)
  let tokenize input =
  let split_words = split_input input in
  List.map classify_token split_words
  
  (* entering a Input and displaying the output *)
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
  
  