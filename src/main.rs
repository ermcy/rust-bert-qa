use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use std::io::*;
fn main(){
  let model = QuestionAnsweringModel::new(Default::default());
  let mut input_1 = String::new();
  let mut input_2 = String::new();
  println!("Input a question.");
  stdin().read_line(&mut input_1).unwrap();
  println!("Input an answer.");
  stdin().read_line(&mut input_2).unwrap();
  input_1 = input_1.trim().parse().unwrap();
  input_2 = input_2.trim().parse().unwrap();
  println!("just a sec");
  let output = model.unwrap().predict(&[QaInput{ question: input_1, context: input_2 }], 10, 32);
  println!("{:#?}", output);
}
