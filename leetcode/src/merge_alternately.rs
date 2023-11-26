pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result: Vec<char> = Vec::new();

    let longer_wordlength = if word1.len() >= word2.len() { word1.len() } else { word2.len() };

    let mut w1_chars =  word1.chars();
    let mut w2_chars =  word2.chars();
    
    for _i in 0..longer_wordlength {
        if let Some(c) =w1_chars.next() {
            result.push(c);
        };

        if  let Some(c) = w2_chars.next() {
            result.push(c);
        }
    }

    return result.into_iter().collect();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_alternately_test_1() {
        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();
        assert_eq!(merge_alternately(word1, word2), "apbqrs".to_string());
    }
}