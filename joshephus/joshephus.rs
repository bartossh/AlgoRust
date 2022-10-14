/// solver for Joshephus problem
/// Flavius Josephus, a Jewish-Roman historian from the first century,
/// tells the story like this: A company of 40 soldiers, along with Josephus himself,
/// were trapped in a cave by Roman soldiers during the Siege of Yodfat in 67 A.D.
/// The Jewish soldiers chose to die rather than surrender,
/// so they devised a system to kill off each other until only one person remained.
/// (That last person would be the only one required to die by their own hand.)

/// All 41 people stood in a circle.
/// The first soldier killed the man to his left, the next surviving soldier killed the man to his left, and so on.
/// Josephus was among the last two men standing, "whether we must say it happened so by chance,
/// or whether by the providence of God," and he convinced the other survivor to surrender rather than die.
pub fn solver(num: u64) -> u64 {
    let mask = 1_u64 << (index_of_highest_bit(num) -1);
    let res = mask ^ num;
    (res << 1) + 1
}

fn index_of_highest_bit(mut n: u64) -> u64 {
    let mut count: u64 = 0;
    while n != 0 {
        n = n >> 1;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::solver;
    #[test]
    fn test_solve_joshephus() {
        let nums = vec![41, 1, 2, 14, 7, 8];
        let expected = vec![19, 1, 1, 13, 7, 1];
        for i in 0..nums.len() {
            let res = solver(nums[i]);
            assert!(res == expected[i]);
        }
    }
}
