use std::ops::Add;

use rand::{distributions::uniform::SampleUniform, Rng};

// rand 获取[min,max)随机数
pub fn rand_range<T>(min: T, max: T) -> T
where
    T: SampleUniform + PartialOrd,
{
    rand::thread_rng().gen_range(min..max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_range() {
        let i = 100;
        for _ in 0..i {
            let a = rand_range(0, 5);
            println!("{}", a);
            assert!(a >= 0 && a < 5);
        }
    }
}
