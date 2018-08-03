extern crate wordy;

use wordy::*;

#[test]
fn simple_answer_plus1() {
    let command = "What is 5 plus 13?";
    assert_eq!(Ok(18), WordProblem::new(command).answer())
}

#[test]
fn simple_answer_plus2() {
    let command = "What is 5 plus 13 plus -6?";
    assert_eq!(Ok(12), WordProblem::new(command).answer())
}

#[test]
fn simple_answer_plus3() {
    let command = "What is 5 plus 13 plus -6 plus -15?";
    assert_eq!(Ok(-3), WordProblem::new(command).answer())
}

#[test]
fn simple_answer_minus() {
    let command = "What is 5 minus 4?";
    assert_eq!(Ok(1), WordProblem::new(command).answer())
}

#[test]
fn simple_answer_minus2() {
    let command = "What is 10 minus 5 minus 2?";
    assert_eq!(Ok(3), WordProblem::new(command).answer())
}

#[test]
fn simple_answer_minus3() {
    let command = "What is 10 minus 5 minus 2 minus 1?";
    assert_eq!(Ok(2), WordProblem::new(command).answer())
}

#[test]
fn simple_answer_multiply() {
    let command = "What is 5 multiplied by 10?";
    assert_eq!(Ok(50), WordProblem::new(command).answer())
}

#[test]
fn simple_answer_divided() {
    let command = "What is 10 divided by 5?";
    assert_eq!(Ok(2), WordProblem::new(command).answer())
}

