// pub fn kebob_to_title(kebob: &str) -> String {
//   kebob
//       .split('-')
//       .map(|word| {
//           let mut chars = word.chars();
//           match chars.next() {
//               None => String::new(),
//               Some(c) => c.to_uppercase().chain(chars).collect(),
//           }
//       })
//       .collect::<Vec<String>>()
//       .join(" ")
// }
