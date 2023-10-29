use std::collections::HashMap;

use super::builder::{MetaBuilder, Builder};

pub struct DefaultMetaBuilder{}

impl MetaBuilder for DefaultMetaBuilder{
    type Item = DefaultBuilder;

    fn new(&self, typename: String, argument: String) -> Self::Item {
        DefaultBuilder::new(typename, argument)
    }
}

pub struct DefaultBuilder{
	typename : String,
	content : String,
	params : HashMap<String, String>,
	children : Vec<DefaultItem>
}

impl DefaultBuilder{
	pub fn new(typename : String, content : String) -> Self{
		Self{ typename, content, params : HashMap::new(), children : vec![] }
	}
}

impl Builder for DefaultBuilder{
    type Item = DefaultItem;

    fn set_param(&mut self, param_name: String, content: String) {
        self.params.insert(param_name, content);
    }

    fn set_child(&mut self, child: Self::Item) {
        todo!()
    }

    fn finish(self) -> Self::Item {
        todo!()
    }
}

pub struct DefaultItem{
	typename : String,
	content : String,
	params : HashMap<String, String>,
	children : Vec<DefaultItem>
}