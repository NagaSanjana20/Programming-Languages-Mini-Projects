use sqlparser::ast::{Expr, Ident, Query, Select, SelectItem, SetExpr, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::collections::HashMap;
use std::io::{self, Write};
use maplit::hashmap;

type Row = HashMap<String, String>;

#[derive(Debug)]
struct Table {
    name: String,
    rows: Vec<Row>,
}

fn evaluate_where_clause(expr: &Expr, row: &Row) -> bool {
    match expr {
        Expr::BinaryOp { left, op, right } if op.to_string().to_uppercase() == "AND" => {
            evaluate_where_clause(left, row) && evaluate_where_clause(right, row)
        }
        Expr::BinaryOp { left, op, right } => {
            let left_val = match &**left {
                Expr::Identifier(Ident { value, .. }) => row.get(value).cloned().unwrap_or_default(),
                _ => return false,
            };
            let right_val = match &**right {
                Expr::Value(sqlparser::ast::Value::SingleQuotedString(s)) => s.clone(),
                _ => return false,
            };
            match op.to_string().as_str() {
                "=" => left_val == right_val,
                _ => false,
            }
        }
        Expr::Nested(e) => evaluate_where_clause(e, row),
        _ => false,
    }
}

fn execute_query(sql: &str, table: &Table) -> Vec<Row> {
    let dialect = GenericDialect;
    let ast = Parser::parse_sql(&dialect, sql).expect("Failed to parse SQL");
    let Statement::Query(query) = &ast[0] else {
        panic!("Only SELECT queries are supported");
    };

    let Query { body, .. } = &**query;
    let SetExpr::Select(select) = &**body else {
        panic!("Only SELECT body supported");
    };

    let Select {
        projection,
        selection,
        from,
        ..
    } = &**select;

    let table_name = &from[0].relation.to_string();
    assert_eq!(
        table_name.to_lowercase(),
        table.name.to_lowercase(),
        "Unknown table"
    );

    table
        .rows
        .iter()
        .filter_map(|row| {
            if let Some(expr) = selection {
                if !evaluate_where_clause(expr, row) {
                    return None;
                }
            }

            let mut projected = Row::new();
            for item in projection {
                match item {
                    SelectItem::UnnamedExpr(Expr::Identifier(ident)) => {
                        if let Some(v) = row.get(&ident.value) {
                            projected.insert(ident.value.clone(), v.clone());
                        }
                    }
                    SelectItem::Wildcard(_) => {
                        projected = row.clone();
                    }
                    _ => {}
                }
            }
            Some(projected)
        })
        .collect()
}

fn is_valid_query_structure(query: &str) -> bool {
    let lower = query.to_lowercase();
    lower.contains("select") && lower.contains("from")
}

fn main() {
    let student_table = Table {
        name: "student".to_string(),
        rows: vec![
            hashmap! {"id".into() => "1".into(), "name".into() => "Alice".into(), "major".into() => "CS".into()},
            hashmap! {"id".into() => "2".into(), "name".into() => "Bob".into(), "major".into() => "Math".into()},
            hashmap! {"id".into() => "3".into(), "name".into() => "Charlie".into(), "major".into() => "CS".into()},
        ],
    };

    println!("Enter SQL query:");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input_query = String::new();
    io::stdin()
        .read_line(&mut input_query)
        .expect("Failed to read input");
    let input_query = input_query.trim();

    if !is_valid_query_structure(input_query) {
        println!("\nQuery is incorrect.");
        return;
    }

    let actual = execute_query(input_query, &student_table);

    println!("\nQuery Output:");
    for row in &actual {
        println!("{:?}", row);
    }

    if !actual.is_empty() {
        println!("\nQuery is correct.");
    } else {
        println!("\nQuery is incorrect.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_table() -> Table {
        Table {
            name: "student".to_string(),
            rows: vec![
                hashmap! {"id".into() => "1".into(), "name".into() => "Alice".into(), "major".into() => "CS".into()},
                hashmap! {"id".into() => "2".into(), "name".into() => "Bob".into(), "major".into() => "Math".into()},
                hashmap! {"id".into() => "3".into(), "name".into() => "Charlie".into(), "major".into() => "CS".into()},
            ],
        }
    }
    
    fn query_is_correct(sql: &str) -> bool {
        let sql_lower = sql.to_lowercase();
        if !sql_lower.contains("select") || !sql_lower.contains("from") {
            return false;
        }

        let table = get_sample_table();
        let parsed = std::panic::catch_unwind(|| execute_query(sql, &table));
        match parsed {
            Ok(rows) => !rows.is_empty(),
            Err(_) => false, // e.g., syntax errors, invalid table name
        }
    }

    #[test]
    fn test_valid_queries() {
        assert!(query_is_correct("SELECT name FROM student WHERE name = 'Alice'"));
        assert!(query_is_correct("SELECT name FROM student WHERE major = 'CS'"));
        assert!(query_is_correct("SELECT name FROM student WHERE major = 'CS' AND id = '3'"));
        assert!(query_is_correct("SELECT * FROM student WHERE id = '2'"));
    }

    #[test]
    fn test_invalid_queries() {
        assert!(!query_is_correct("SELECT name student WHERE major = 'CS'")); // missing FROM
        assert!(!query_is_correct("name FROM student WHERE major = 'CS'"));    // missing SELECT
        assert!(!query_is_correct("SELECT name FROM students WHERE name = 'Alice'")); // wrong table
        assert!(!query_is_correct("WHERE name = 'Alice'")); // missing SELECT and FROM
    }
}
