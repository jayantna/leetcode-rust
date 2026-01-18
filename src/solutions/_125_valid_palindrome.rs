pub struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut ans = true;
        let filtered: String = s.chars().filter(|x| x.is_alphanumeric()).map(|x| x.to_ascii_lowercase()).collect::<String>();
        filtered == filtered.chars().rev().collect::<String>()
    }
    pub fn run () {
        let s = String::from("A man, a plan, a canal: Panama");
        let result = Solution::is_palindrome(s);
        println!("{:?}", result);
    }
}