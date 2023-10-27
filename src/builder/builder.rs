pub trait MetaBuilder<B, T>
where
    B: Builder<T>,
{
    fn new(typename: String, content: String) -> B;
}

pub trait Builder<T> {
    fn set_param(&mut self, param_name: String, content: String);
    fn set_child(&mut self, child: T);
    fn finish(self) -> T;
}
