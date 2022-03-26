pub mod ip_utils {
    const OCTETS_IN_IP: usize = 4;
    pub fn is_subnet_mask_valid(ip_string: &str) -> bool {
        let ip: u32;
        match convert_str_to_ip(ip_string) {
            Ok(x) => {
                ip = x;
            }
            Err(_e) => {
                return false;
            }
        }
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
        num & (1 << bit) != 0
    }

    fn is_ip_valid(ip_string: &str) -> bool {
        let v: Vec<&str> = ip_string.split('.').collect();
        if v.len() != OCTETS_IN_IP {
            return false;
        }
        for item in v.iter().take(OCTETS_IN_IP) {
            if item.parse::<u32>().is_err() {
                return false;
            }
        }
        true
    }

    fn convert_str_to_ip(ip_string: &str) -> Result<u32, ()> {
        if !is_ip_valid(ip_string) {
            return Err(());
        }
        let v: Vec<&str> = ip_string.split('.').collect();
        let mut nums = [0; OCTETS_IN_IP];
        for i in 0..OCTETS_IN_IP {
            nums[i] = v[i].parse::<u32>().unwrap_or(0);
        }
        let ip: u32 = nums[3] | nums[2] << 8 | nums[1] << 16 | nums[0] << 24;

        Ok(ip)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_bit_set() {
            assert_eq!(is_bit_set(0xff, 7), true);
            assert_eq!(is_bit_set(0b1001, 0), true);
            assert_eq!(is_bit_set(0b1001, 1), false);
        }

        #[test]
        fn test_convert_str_to_ip() {
            assert_eq!(
                convert_str_to_ip("192.168.1.0").unwrap(),
                (192 << 24) | (168 << 16) | (1 << 8) | 0
            );
        }

        #[test]
        fn test_is_ip_valid() {
            assert_eq!(is_ip_valid("192.168.1.0"), true);
            assert_eq!(is_ip_valid("255.168.1.0"), true);
            assert_eq!(is_ip_valid("192.168.1.0.2"), false);
            assert_eq!(is_ip_valid("192.168.1.a"), false);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_is_subnet_mask_valid() {
        assert_eq!(crate::ip_utils::is_subnet_mask_valid("255.255.255.0"), true);
        assert_eq!(crate::ip_utils::is_subnet_mask_valid("255.255.192.0"), true);
        assert_eq!(crate::ip_utils::is_subnet_mask_valid("255.0.0.0"), true);

        assert_eq!(
            crate::ip_utils::is_subnet_mask_valid("255.255.100.0"),
            false
        );
        assert_eq!(crate::ip_utils::is_subnet_mask_valid("0.255.255.0"), false);
    }
}
