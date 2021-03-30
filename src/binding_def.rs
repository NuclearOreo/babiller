use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, Self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div
                    },
                },
            ),
        );
    }
}
