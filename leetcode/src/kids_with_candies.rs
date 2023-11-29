pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut result : Vec<bool> = Vec::new();
    let max = which_is_greatest(&candies);

    for i in 0..candies.len() {
        if candies.get(i).unwrap() + extra_candies >= max {
            result.push(true);
        } else {
            result.push(false);
        }
    }

    return result;
}

pub fn which_is_greatest(candies: &Vec<i32>) -> i32 {
    let mut prev_value = candies.get(0).unwrap().to_owned();
    let mut max = prev_value;

    for i in 1..candies.len() {
        if candies.get(i).unwrap().to_owned() > prev_value { 
            max = candies.get(i).unwrap().to_owned();
            prev_value = max;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kids_with_candies_test_1() {
        let candies = vec!(2,3,5,1,3);
        let extra_candies = 3;
        assert_eq!(kids_with_candies(candies, extra_candies), vec!(true,true,true,false,true));
    }
}