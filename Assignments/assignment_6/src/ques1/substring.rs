/// Substring function is used to find all substring of input string
///
/// #Arguments
///
/// input_string - Taking String as input and generating substring of input string
///
/// #Return
///
/// Returns a vector<Sting> having all substring of input string
pub fn substring(input: String) -> Vec<String> {
    if input.is_empty() {
        return vec!["".to_string()];
    }
    let mut new_string: Vec<String> = Vec::new();
    let mut string_name: &str;
    for index_1 in 0..input.len() {
        for index_2 in index_1..input.len() {
            string_name = &input[index_1..(index_2 + 1)];
            new_string.push(string_name.to_string());
        }
    }
    new_string
}