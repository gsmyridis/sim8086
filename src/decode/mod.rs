pub mod error;
pub mod fields;
pub mod instr;
pub mod operand;
pub mod ops;
pub mod address;



// use error::{PResult, ParsingError};
// 
// /// Returns a reference to the `n` first bytes an 
// pub fn take(n: usize, i: &[u8]) -> PResult<&[u8], &[u8]> {
//     if i.len() < n {
//         Err(ParsingError::Incomplete(n - i.len()))
//     } else {
//         Ok((&i[..n], &i[n..]))
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn test_take_success_0() {
//         let buffer: &[u8] = &[0, 1, 2, 3, 4, 5];
//         let (taken, rest) = take(0, buffer).unwrap();
//         assert!(taken.is_empty());
//         assert_eq!(rest, [0, 1, 2, 3, 4, 5]);
//     }
// 
//     #[test]
//     fn test_take_success_1() {
//         let buffer: &[u8] = &[0, 1, 2, 3, 4, 5];
//         let (taken, rest) = take(3, buffer).unwrap();
//         assert_eq!(taken, [0, 1, 2]);
//         assert_eq!(rest, [3, 4, 5]);
//     }
// 
//     #[test]
//     fn test_take_failure() {
//         let buffer: &[u8] = &[0, 1, 2];
//         assert_eq!(take(4, buffer), Err(ParsingError::Incomplete(1)));
//     }
// }
