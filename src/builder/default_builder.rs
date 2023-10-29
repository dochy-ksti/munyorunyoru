use std::collections::HashMap;

use super::builder::{MetaBuilder, Builder};

pub struct DefaultMetaBuilder{}

impl MetaBuilder for DefaultMetaBuilder{
    type Item = DefaultBuilder;

    fn build(&self, typename: String, argument: String) -> Result<Self::Item, String> {
        Ok(DefaultBuilder::new(typename, argument))
    }
}

pub struct DefaultBuilder{
	typename : String,
	content : String,
	params : HashMap<String, String>,
	children : Vec<DefaultItem>
}

impl DefaultBuilder{
	pub fn new(typename : String, argument : String) -> Self{
		Self{ typename, content: argument, params : HashMap::new(), children : vec![] }
	}
}

impl Builder for DefaultBuilder{
    type Item = DefaultItem;

    fn set_param(&mut self, param_name: String, argument: String) -> Result<(), String>{
        let b = self.params.insert(param_name, argument);
        if let Some(param_name) = b{
            return Err(format!("{param_name} is applied multiple times for {} {}", &self.typename, &self.content));
        }
        Ok(())
    }

    fn set_child(&mut self, child: Self::Item) -> Result<(), String>{
        self.children.push(child);
        Ok(())
    }

    fn finish(self) -> Result<Self::Item, String> {
        Ok(Self::Item{ 
            typename : self.typename, 
            content : self.content,
            params : self.params,
            children : self.children
         })
    }
}

pub struct DefaultItem{
	pub typename : String,
	pub content : String,
	pub params : HashMap<String, String>,
	pub children : Vec<DefaultItem>
}