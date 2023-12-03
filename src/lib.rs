use std::i32;

pub fn quersumme(mut num: i32) -> i32 {
    while num > 9 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        num = sum;
    }
    num
}



#[cfg(test)]
mod tests {

    use crate::quersumme;

    #[test]
    fn test_qs123() {
        let res = quersumme(123);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_qs4321() {
        let res = quersumme(4321);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_qs1973() {
        let res = quersumme(1973);
        assert_eq!(res, 2);
    }

   
}
