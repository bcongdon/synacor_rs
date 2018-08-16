enum_from_primitive! {
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OP {
    Halt = 0,
    Set,
    Push,
    Pop,
    Eq,
    Gt,
    Jmp,
    Jt,
    Jf,
    Add,
    Mul,
    Modulo,
    And,
    Or,
    Not,
    Rmem,
    Wmem,
    Call,
    Ret,
    Out,
    Input,
    NoOp
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::FromPrimitive;

    #[test]
    fn test_values() {
        assert_eq!(OP::from_u16(0), Some(OP::Halt));
        assert_eq!(OP::from_u16(21), Some(OP::NoOp));
    }
}
