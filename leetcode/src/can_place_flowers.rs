pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
    let mut i = 0;
    let len = flowerbed.len();

    while i < len {
        if flowerbed[i] == 1 {
            i += 2;
            continue;
        }

        if (i == 0 || flowerbed[i - 1] == 0) && (i == len - 1 || flowerbed[i + 1] == 0) {
            flowerbed[i] = 1;
            n -= 1;
            i += 2;
        } else {
            i += 1;
        }

        if n <= 0 {
            return true;
        }

    }

    return n <= 0;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_place_flowers_test_1() {
        let flowerbed = vec!(1,0,0,0,1);
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn can_place_flowers_test_2() {
        let flowerbed = vec!(1,0,0,0,1);
        let n = 2;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }

    #[test]
    fn can_place_flowers_test_3() {
        let flowerbed = vec!(0,0,1,0,1);
        let n = 2;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }

    #[test]
    fn can_place_flowers_test_4() {
        let flowerbed = vec!(0,0,0,0);
        let n = 3;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }

    #[test]
    fn can_place_flowers_test_5() {
        let flowerbed = vec!(1,0);
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }
}