use crate::calc::bit_width::BitWidth;

#[derive(Clone, Copy, Debug)]
pub struct CalcResult {
    pub value: u64,
    pub overflow: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,
}

impl Operator {
    pub fn symbol(&self) -> &'static str {
        match self {
            Operator::And => "&",
            Operator::Or => "|",
            Operator::Xor => "^",
            Operator::Not => "~",
            Operator::ShiftLeft => "<<",
            Operator::ShiftRight => ">>",
        }
    }
}

pub fn apply(op: Operator, a: u64, b: u64, width: &BitWidth) -> CalcResult {
    let mask = width.mask();
    let bits = width.num_bits();
    let (value, overflow) = match op {
        Operator::And => (a & b, false),
        Operator::Or => (a | b, false),
        Operator::Xor => (a ^ b, false),
        Operator::Not => (!a, false),
        Operator::ShiftLeft => {
            let shift = b.min(bits as u64);
            let overflow = b >= bits as u64;
            (a << shift, overflow)
        }
        Operator::ShiftRight => {
            let shift = b.min(bits as u64);
            let overflow = b >= bits as u64;
            (a >> shift, overflow)
        }
    };
    CalcResult {
        value: value & mask,
        overflow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        let w = BitWidth::B8;
        assert_eq!(apply(Operator::And, 0xFF, 0x0F, &w).value, 0x0F);
        assert_eq!(apply(Operator::And, 0xF0, 0x0F, &w).value, 0x00);
    }

    #[test]
    fn test_or() {
        let w = BitWidth::B8;
        assert_eq!(apply(Operator::Or, 0xF0, 0x0F, &w).value, 0xFF);
        assert_eq!(apply(Operator::Or, 0x01, 0x02, &w).value, 0x03);
    }

    #[test]
    fn test_xor() {
        let w = BitWidth::B8;
        assert_eq!(apply(Operator::Xor, 0xFF, 0x0F, &w).value, 0xF0);
        assert_eq!(apply(Operator::Xor, 0x0F, 0x0F, &w).value, 0x00);
    }

    #[test]
    fn test_not_masked() {
        let w = BitWidth::B8;
        assert_eq!(apply(Operator::Not, 0x00, 0, &w).value, 0xFF);
        assert_eq!(apply(Operator::Not, 0xFF, 0, &w).value, 0x00);
        assert_eq!(apply(Operator::Not, 0xAA, 0, &w).value, 0x55);
    }

    #[test]
    fn test_not_32bit() {
        let w = BitWidth::B32;
        assert_eq!(apply(Operator::Not, 0x00000000, 0, &w).value, 0xFFFFFFFF);
        assert_eq!(apply(Operator::Not, 0xFFFFFFFF, 0, &w).value, 0x00000000);
    }

    #[test]
    fn test_shift_left() {
        let w = BitWidth::B32;
        assert_eq!(apply(Operator::ShiftLeft, 1, 8, &w).value, 256);
        assert_eq!(apply(Operator::ShiftLeft, 0x01, 4, &w).value, 0x10);
    }

    #[test]
    fn test_shift_right() {
        let w = BitWidth::B32;
        assert_eq!(apply(Operator::ShiftRight, 256, 8, &w).value, 1);
        assert_eq!(apply(Operator::ShiftRight, 0xF0, 4, &w).value, 0x0F);
    }

    #[test]
    fn test_shift_overflow_8bit() {
        let w = BitWidth::B8;
        // Shifting by >= 8 bits should overflow and result in 0
        let result = apply(Operator::ShiftLeft, 0xFF, 8, &w);
        assert!(result.overflow);
        assert_eq!(result.value, 0);
    }

    #[test]
    fn test_shift_overflow_32bit() {
        let w = BitWidth::B32;
        let result = apply(Operator::ShiftRight, 1, 32, &w);
        assert!(result.overflow);
        assert_eq!(result.value, 0);
    }

    #[test]
    fn test_64bit_all_ones() {
        let w = BitWidth::B64;
        let result = apply(Operator::Not, 0, 0, &w);
        assert_eq!(result.value, 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn test_16bit_truncation() {
        let w = BitWidth::B16;
        let result = apply(Operator::Or, 0xFFFF, 0x0001, &w);
        assert_eq!(result.value, 0xFFFF);
    }

    #[test]
    fn test_operator_symbol() {
        assert_eq!(Operator::And.symbol(), "&");
        assert_eq!(Operator::Or.symbol(), "|");
        assert_eq!(Operator::Xor.symbol(), "^");
        assert_eq!(Operator::Not.symbol(), "~");
        assert_eq!(Operator::ShiftLeft.symbol(), "<<");
        assert_eq!(Operator::ShiftRight.symbol(), ">>");
    }
}
