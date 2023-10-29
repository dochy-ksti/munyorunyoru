pub trait MetaBuilder
where
    Self::Item: Builder,
{
    type Item;

    fn new(&self, typename: String, argument: String) -> Self::Item;
}

pub trait Builder {
    type Item;
    fn set_param(&mut self, param_name: String, content: String);
    fn set_child(&mut self, child: Self::Item);
    fn finish(self) -> Self::Item;
}
