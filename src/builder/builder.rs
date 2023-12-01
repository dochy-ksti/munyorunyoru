/// This trait is not meant for general use
pub trait MetaBuilder
{
	/// Builder which is created by this meta_builder.
    type Item;

	/// Build a builder
    fn build(&self, typename: String, argument: String) -> Result<Self::Item, String>;
}

/// This trait is not meant for general use
pub trait Builder {
	/// The item this builder builds
    type Item;
	/// Set param
    fn set_param(&mut self, param_name: String, argument: String) -> Result<(), String>;
	/// Set child
    fn set_child(&mut self, child: Self::Item) -> Result<(), String>;
	/// Create the Item
    fn finish(self) -> Result<Self::Item, String>;
}
