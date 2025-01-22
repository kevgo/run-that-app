use crate::applications::App;

pub struct ExecutableCall {
  app: Box<dyn App>,
  args: Vec<String>,
}
