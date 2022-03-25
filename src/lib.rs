pub mod ip_utils {
    const OCTETS_IN_IP: usize = 4;
    pub fn is_subnet_mask_valid(ip_string: &str) -> bool {
        let v: Vec<&str> = ip_string.split('.').collect();
        if v.len() != OCTETS_IN_IP {
            return false;
        }
        let mut nums = [0; OCTETS_IN_IP];
        for i in 0..OCTETS_IN_IP {
            nums[i] = v[i].parse::<u32>().unwrap_or(0);
        }
        let ip: u32 = nums[3] | nums[2] << 8 | nums[1] << 16 | nums[0] << 24;

        for first_search_index in 0..32 {
            if is_bit_set(ip, first_search_index) {
                for next_search_index in first_search_index + 1..31 {
                    if !is_bit_set(ip, next_search_index) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_bit_set(num: u32, bit: u8) -> bool {
        num | (1 << bit) == num
    }
}
