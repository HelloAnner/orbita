use crate::brute_force::Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let week = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
        let month_days = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];
        let mut days = (year - 1971) * 365 + (year - 1969) / 4;
        days += &month_days[..month as usize - 1].iter().sum();
        if (year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)) && month >= 3 {
            days += 1
        }
        days += day;
        week[(days as usize + 3) % 7].to_owned()
    }
}

#[test]
pub fn test_it() {}