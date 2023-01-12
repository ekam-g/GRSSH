// let mut return_val: Vec<Vec<String>> = vec![];
// let mut word_letter: usize = 0;
// let mut quote = false;
// for letter in what.split("") {
//     quote = handle_quote(letter, quote, "\"");
//     quote = handle_quote(letter, quote, "'");
//     if letter == " " || letter == "\n" {
//         if !quote {
//             word_letter += 1;
//         } else {
//             return_val.push(vec![]);
//             return_val[word_letter].push(letter.to_owned());
//         }
//     } else if !letter.is_empty() && letter != "\"" {
//         return_val.push(vec![]);
//         return_val[word_letter].push(letter.to_owned());
//     }
// }
// let mut return_vec = vec![];
// for word in return_val {
//     if !word.is_empty() {
//         return_vec.push(word.join(""))
//     }
// }
// return_vec

//fn handle_quote(letter: &str, quote: bool, compare: &str) -> bool {
//     if letter == compare {
//         return !quote;
//     }
//     quote
// }