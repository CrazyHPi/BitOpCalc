#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BitWidth {
    B8,
    B16,
    B32,
    B64,
}

impl BitWidth {
    pub fn mask(&self) -> u64 {
        match self {
            BitWidth::B8 => 0xFF,
            BitWidth::B16 => 0xFFFF,
            BitWidth::B32 => 0xFFFFFFFF,
            BitWidth::B64 => 0xFFFFFFFFFFFFFFFF,
        }
    }

    pub fn num_bits(&self) -> usize {
        match self {
            BitWidth::B8 => 8,
            BitWidth::B16 => 16,
            BitWidth::B32 => 32,
            BitWidth::B64 => 64,
        }
    }

    pub fn signed_value(&self, raw: u64) -> i64 {
        match self {
            BitWidth::B8 => raw as i8 as i64,
            BitWidth::B16 => raw as i16 as i64,
            BitWidth::B32 => raw as i32 as i64,
            BitWidth::B64 => raw as i64,
        }
    }

    pub fn hex_digits(&self) -> usize {
        self.num_bits() / 4
    }

    pub fn format_binary(&self, value: u64) -> String {
        let bits = self.num_bits();
        let mut result = String::with_capacity(bits + bits / 4);
        for i in (0..bits).rev() {
            let bit = (value >> i) & 1;
            result.push(if bit == 1 { '1' } else { '0' });
            if i > 0 && i % 4 == 0 {
                result.push(' ');
            }
        }
        result
    }

    pub fn format_hex(&self, value: u64) -> String {
        format!("0x{:0width$X}", value, width = self.hex_digits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(BitWidth::B8.mask(), 0xFF);
        assert_eq!(BitWidth::B16.mask(), 0xFFFF);
        assert_eq!(BitWidth::B32.mask(), 0xFFFFFFFF);
        assert_eq!(BitWidth::B64.mask(), 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn test_num_bits() {
        assert_eq!(BitWidth::B8.num_bits(), 8);
        assert_eq!(BitWidth::B16.num_bits(), 16);
        assert_eq!(BitWidth::B32.num_bits(), 32);
        assert_eq!(BitWidth::B64.num_bits(), 64);
    }

    #[test]
    fn test_signed_value_8bit() {
        assert_eq!(BitWidth::B8.signed_value(0), 0);
        assert_eq!(BitWidth::B8.signed_value(127), 127);
        assert_eq!(BitWidth::B8.signed_value(128), -128);
        assert_eq!(BitWidth::B8.signed_value(255), -1);
    }

    #[test]
    fn test_signed_value_32bit() {
        assert_eq!(BitWidth::B32.signed_value(0), 0);
        assert_eq!(BitWidth::B32.signed_value(0x7FFFFFFF), 2147483647);
        assert_eq!(BitWidth::B32.signed_value(0xFFFFFFFF), -1);
    }

    #[test]
    fn test_format_binary_8bit() {
        assert_eq!(BitWidth::B8.format_binary(0), "0000 0000");
        assert_eq!(BitWidth::B8.format_binary(255), "1111 1111");
        assert_eq!(BitWidth::B8.format_binary(0xAA), "1010 1010");
    }

    #[test]
    fn test_format_binary_32bit() {
        let s = BitWidth::B32.format_binary(0xFF);
        assert!(s.contains("1111 1111"));
        assert_eq!(s.len(), 32 + 7); // 32 bits + 7 spaces
    }

    #[test]
    fn test_format_hex() {
        assert_eq!(BitWidth::B8.format_hex(0xFF), "0xFF");
        assert_eq!(BitWidth::B16.format_hex(0xFF), "0x00FF");
        assert_eq!(BitWidth::B32.format_hex(0xFF), "0x000000FF");
        assert_eq!(BitWidth::B64.format_hex(0xFF), "0x00000000000000FF");
    }

    #[test]
    fn test_hex_digits() {
        assert_eq!(BitWidth::B8.hex_digits(), 2);
        assert_eq!(BitWidth::B16.hex_digits(), 4);
        assert_eq!(BitWidth::B32.hex_digits(), 8);
        assert_eq!(BitWidth::B64.hex_digits(), 16);
    }
}
