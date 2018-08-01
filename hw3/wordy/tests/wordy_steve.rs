extern crate wordy;

use wordy::*;

#[test]
fn simple_answer() {
    let command = "What is 5 plus 13 plus 6 plus 15?";
    assert_eq!(39, wordy::simple_answer(command));
}

#[test]
fn simple_answer1() {
    let command = "What is 5 plus 13 plus 6?";
    assert_eq!(24, wordy::simple_answer(command));
}
