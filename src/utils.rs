use rand::{distributions::uniform::SampleUniform, Rng};

// rand 获取[min,max)随机数
pub fn rand_range<T>(min: T, max: T) -> T
where
    T: SampleUniform + PartialOrd,
{
    rand::thread_rng().gen_range(min..max)
}

// 获取min值
pub fn min<T>(range: T) -> T::Item
where
    T: Iterator,
    T::Item: Ord,
{
    range.min().unwrap()
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

    #[test]
    fn test_min() {
        let b = vec![3, 45, 56, 1];
        let a = min(b.iter());
        assert_eq!(1, *a);
    }
}
