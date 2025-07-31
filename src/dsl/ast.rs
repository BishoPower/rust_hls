#[derive(Clone, Debug)]
pub enum Expr {
    Input { name: String, width: u32 },
    Const { value: i32, width: u32 },
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Output { name: String, expr: Box<Expr> },
}

// DSL constructor helpers
pub fn input<T: Into<String>>(name: T, width: u32) -> Expr {
    Expr::Input { name: name.into(), width }
}

pub fn const_val(value: i32, width: u32) -> Expr {
    Expr::Const { value, width }
}

pub fn add(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Add(Box::new(lhs), Box::new(rhs))
}

pub fn sub(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Sub(Box::new(lhs), Box::new(rhs))
}

pub fn mul(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Mul(Box::new(lhs), Box::new(rhs))
}

pub fn output<T: Into<String>>(name: T, expr: Expr) -> Expr {
    Expr::Output { name: name.into(), expr: Box::new(expr) }
}
