pub type Program = Vec<Instruction>;

/// Logo instructions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    /// fd / forward
    Forward(Expr),
    /// bk / back
    Backward(Expr),
    /// lt / left
    Left(Expr),
    /// rt / right
    Right(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Expr {
    Number(f64),
}
