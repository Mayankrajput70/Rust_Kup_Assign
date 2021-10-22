#[cfg(test)]
mod test {
    #[test]
    fn substring_test() {
        use crate::ques1::substring::substring;
        assert_eq!(
            substring("Done".to_string()),
            ["D", "Do", "Don", "Done", "o", "on", "one", "n", "ne", "e"]
        );
    }
    #[test]
    fn substring_test_2(){
        use crate::ques1::substring::substring;
        assert_ne!(
            substring("One".to_string()),
            ["O", "nO", "enO", "n", "en", "e"]
        );
    }

    #[test]
    fn pattern_test() {
        use crate::ques1::pattern_search::pattern;
        assert_eq!(
            pattern("Mayank Singh".to_string(), "ing".to_string()),
            "8".to_string()
        );
    }
    #[test]
    fn pattern_test_2() {
        use crate::ques1::pattern_search::pattern;
        assert_ne!(
            pattern("rust program".to_string(), "ro".to_string()),
            "no match".to_string()
        );
    }

    #[test]
    fn check_string() {
        use crate::ques2::find_string::desired_output;
        assert_eq!(desired_output("jjdhid", "ikjhjk", "rtysgi"), "itdsgk");
    }
    #[test]
    fn check_string_2() {
        use crate::ques2::find_string::desired_output;
        assert_ne!(desired_output("abcd", "efgh", "ijkl"), "afkl");
    }
}