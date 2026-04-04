pub fn shift_left_with_carry(value: u8, shift: u8) -> (u8, u8) {
    /*
        se 0b_1000_0000 << 2 então:
            - result = 0b_0000_0000
            - carry  = 0b_0000_0010

        se 0b_0000_0001 << 32 então:
            - result = 0b_0000_0000
            - carry  = 0b_0000_0000
    */
    if shift == 0 {
        return (value, 0);
    }

    if shift >= 8 {
        let carry = (value as u16).checked_shl(shift as u32).unwrap_or(0) >> 8;
        return (0, carry as u8);
    }

    let result = value << shift;
    let carry = value >> (8 - shift);
    (result, carry)
}

pub fn shift_right_with_carry(value: u8, shift: u8) -> (u8, u8) {
    /*
        se 0b_0000_1111 >> 3 então:
            - result = 0b_0000_0001
            - carry  = 0b_1110_0000

        se 0b_0000_0001 >> 32 então:
            - result = 0b_0000_0000
            - carry  = 0b_0000_0000
    */
    if shift == 0 {
        return (value, 0);
    }

    if shift >= 8 {
        return (0, 0);
    }

    let result = value >> shift;
    let carry = (value & ((1 << shift) - 1)) << (8 - shift);
    (result, carry)
}