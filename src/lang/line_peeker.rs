pub(crate) struct LinePeeker{
    iter : <Vec<String> as IntoIterator>::IntoIter,
    prev : Option<String>,
    current : Option<String>,
}

impl LinePeeker{
    pub(crate) fn new(iter : <Vec<String> as IntoIterator>::IntoIter) -> LinePeeker{
        LinePeeker{
            iter, prev : None, current : None
        }
    }

    pub(crate) fn next(&mut self) -> Option<&str>{
        self.prev = self.current.take();
        self.current = self.iter.next();
        self.current.as_ref().map(|s| s.as_str())
    }

    pub(crate) fn modify_current(&mut self, modifier : impl Fn(Option<String>, Option<String>) -> Option<String>){
        let prev = self.prev.take();
        let current = self.current.take();
        let new = modifier(prev, current);
        let current = new;
    }

    pub(crate) fn take_prev(&mut self) -> Option<String>{
        self.prev.take()
    }
}