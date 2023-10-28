
pub struct MetaBuilderArguments{
	pub typename : String,
	pub argument : String,
}

impl MetaBuilderArguments {
    pub fn new(typename: String, argument: String) -> Self { Self { typename, argument } }
}

pub trait Builder<T> {
    fn set_param(&mut self, param_name: String, content: String);
    fn set_child(&mut self, child: T);
    fn finish(self) -> T;
}
