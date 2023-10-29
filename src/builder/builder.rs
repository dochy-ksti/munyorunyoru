pub trait MetaBuilder
where
    Self::Item: Builder,
{
    type Item;

    fn new(&self, typename: String, argument: String) -> Result<Self::Item, String>;
}

pub trait Builder {
    type Item;
    fn set_param(&mut self, param_name: String, content: String) -> Result<(), String>;
    fn set_child(&mut self, child: Self::Item) -> Result<(), String>;
    fn finish(self) -> Result<Self::Item, String>;
}
