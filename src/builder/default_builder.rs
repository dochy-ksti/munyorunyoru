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
	pub fn new(typename : String, content : String) -> Self{
		Self{ typename, content, params : HashMap::new(), children : vec![] }
	}
}

impl Builder for DefaultBuilder{
    type Item = DefaultItem;

    fn set_param(&mut self, param_name: String, content: String) -> Result<(), String>{
        let _b = self.params.insert(param_name, content);
        if _b.is_some(){
            //You can throw an error here, but I don't want to make the default builder picky.
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
	typename : String,
	content : String,
	params : HashMap<String, String>,
	children : Vec<DefaultItem>
}