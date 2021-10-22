/// The function _desired_output is used to give desired output and accepts three string as input
///
/// #Arguments
///
///string_1 - first input string
///string_2 - second input string
///string_3 - third input string
///
/// #Return
///
/// Returns the String to give the desired output....
pub fn _desired_output(str_1: &str, str_2: &str, str_3: &str) -> String {
    let mut position = 0;
    let mut iteration: usize = 0;
    let mut count: Vec<char> = Vec::new();
    while iteration < str_1.len() {
        if position % 2 == 0 {
            let compare_result_1 = if str_1.chars().nth(iteration) < str_2.chars().nth(iteration) {
                str_1.chars().nth(iteration)
            } else {
                str_2.chars().nth(iteration)
            };
            let compare_result_2 = if compare_result_1 < str_3.chars().nth(iteration) {
                compare_result_1
            } else {
                str_3.chars().nth(iteration)
            };
            count.push(compare_result_2.unwrap());
        } else {
            let compare_result_1 = if str_1.chars().nth(iteration) > str_2.chars().nth(iteration) {
                str_1.chars().nth(iteration)
            } else {
                str_2.chars().nth(iteration)
            };
            let compare_result_2 = if compare_result_1 > str_3.chars().nth(iteration) {
                compare_result_1
            } else {
                str_3.chars().nth(iteration)
            };
            count.push(compare_result_2.unwrap());
        }
        iteration += 1;
        position += 1
    }
    let result: String = count.iter().collect();
    result
}
