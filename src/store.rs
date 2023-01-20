pub struct Store {
  pub model_callback: Option<Box<dyn Fn() -> crate::model::Model>>
}