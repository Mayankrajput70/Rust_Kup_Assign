#[cfg(test)]
mod test {
    #[test]
    fn substring_test() {
        use crate::ques1::substring::_substring;
        assert_eq!(
            _substring("Done".to_string()),
            ["D", "Do", "Don", "Done", "o", "on", "one", "n", "ne", "e"]
        );
    }
    #[test]
    fn substring_test_2(){
        use crate::ques1::substring::_substring;
        assert_ne!(
            _substring("One".to_string()),
            ["O", "nO", "enO", "n", "en", "e"]
        );
    }

    #[test]
    fn pattern_test() {
        use crate::ques1::pattern_search::_pattern;
        assert_eq!(
            _pattern("Mayank Singh".to_string(), "ing".to_string()),
            "8".to_string()
        );
    }
    #[test]
    fn pattern_test_2() {
        use crate::ques1::pattern_search::_pattern;
        assert_ne!(
            _pattern("rust program".to_string(), "ro".to_string()),
            "no match".to_string()
        );
    }

    #[test]
    fn check_string() {
        use crate::ques2::find_string::_desired_output;
        assert_eq!(_desired_output("jjdhid", "ikjhjk", "rtysgi"), "itdsgk");
    }
    #[test]
    fn check_string_2() {
        use crate::ques2::find_string::_desired_output;
        assert_ne!(_desired_output("abcd", "efgh", "ijkl"), "afkl");
    }
}