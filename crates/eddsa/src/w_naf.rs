use zkp_u256::U256;

pub fn w_naf_sequence(mut value: U256, width: usize, result: [U256; 8]) -> U256 {
    let a = U256::from(1 << width);
    let b = a.clone() >> 1;
    let mut k_i = U256::from(0);

    let mut res = &result[0] + U256::from_hex_str("0xff");

    while value > U256::from(0) {
        if &value % U256::from(2) > U256::from(0) {
            k_i = &value % &a;
            k_i = &k_i
                - &a * {
                    if k_i > b {
                        0
                    } else {
                        1
                    }
                };
            value = &value - &k_i;
        }
        res = &b + &k_i;
        value /= U256::from(2);
        res = &res - U256::from(1);
    }
    res - &result[0]
}
