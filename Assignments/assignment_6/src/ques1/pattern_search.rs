/// Pattern function is used to search pattern in given input string
///
/// #Arguments
///
/// input_string - Given input string  to Apply to find pattern
/// search_pattern - string to checked with input_string
///
/// #Return
///
/// Returns the String to given value with index and match or not
pub fn pattern(input: String, output: String) -> String {
    let input_vec: Vec<char> = input.chars().collect();
    let output_vec: Vec<char> = output.chars().collect();
    let mut iteration_count = 0;
    let mut pattern_check;
    let mut temp_index;
    for index in 0..(input_vec.len() - output_vec.len() + 1) {
        temp_index = index;
        pattern_check = index;
        for element_match_index in &output_vec {
            if element_match_index == &input_vec[temp_index] {
                iteration_count += 1;
            }
            if iteration_count == output.len() {
                return pattern_check.to_string();
            }
            temp_index += 1;
        }
        iteration_count = 0;
    }
    "no match".to_string()
}
