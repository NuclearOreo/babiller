mod binding_def;
mod expr;
mod stmt;
mod val;

mod env;
mod utils;

struct Parse(stmt::Stmt);

fn parse(s: &str) -> Result<Parse, String> {
    let (s, stmt) = stmt::Stmt::new(s)?;

    if s.is_empty() {
        Ok(Parse(stmt))
    } else {
        Err("input was not consumed fully by parser".to_string())
    }
}
