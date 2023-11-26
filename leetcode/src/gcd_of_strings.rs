pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let (shorter, longer) = if str1.len() >= str2.len() { (str2.as_str(),str1.as_str()) } else { (str1.as_str(),str2.as_str()) };
    
    for i in 0..shorter.len() {
        if is_divisible(&shorter[0..shorter.len() - i], shorter) && is_divisible(&shorter[0..shorter.len() - i], longer) {
            return shorter[0..shorter.len() - i].to_owned();
        }
    }

    return "".to_string();
}

pub fn is_divisible(to_check: &str, mut against: &str) -> bool{
    if against.len() % to_check.len() != 0 { 
        return false; 
    };

    while !against.is_empty() {
        if against.starts_with(to_check) {
            against = against.split_at(to_check.len()).1;
        } else {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_of_strings_test_1() {
        let str1 = "ABABAB".to_string();
        let str2 = "ABAB".to_string();
        assert_eq!(gcd_of_strings(str1, str2), "AB".to_string());
    }
}