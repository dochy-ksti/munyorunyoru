use crate::builder::builder::Builder;

pub(crate) struct TreeItem<B> {
    pub item: B,
    pub children: Vec<TreeItem<B>>,
    pub start_index : usize,
}

impl<B> TreeItem<B> {
    pub(crate) fn new(item: B, start_index : usize) -> Self {
        Self {
            item,
            children: vec![],
            start_index
        }
    }
    pub(crate) fn last_mut(&mut self) -> Option<&mut TreeItem<B>> {
        self.children.last_mut()
    }
}

pub(crate) struct BuilderTree<B> {
    pub root: TreeItem<B>,
}

impl<B> BuilderTree<B> {
    pub(crate) fn new(root: B, start_index : usize) -> Self {
        Self {
            root: TreeItem::new(root, start_index),
        }
    }
    pub(crate) fn add(&mut self, item: B, level: usize, start_index : usize) -> Result<(), String> {
        let mut parent = Some(&mut self.root);
        for i in 0..level {
            parent = parent.unwrap().last_mut();
            if parent.is_none() {
                Err(format!(
                    "The item with indent level {level} is not found. The max level is {i}"
                ))?
            }
        }
        parent.unwrap().children.push(TreeItem::new(item, start_index));
        Ok(())
    }
}

impl<B,T> BuilderTree<B> where B : Builder<Item=T>{
	pub(crate) fn finish(self) -> Result<Vec<T>, String>{
		self.root.children.into_iter().map(|c| Self::make_child(c)).collect()
	}

	fn make_child(item : TreeItem<B>) -> Result<T, String>{
		let children = item.children;
		let mut item = item.item;
		for c in children.into_iter().map(|c| Self::make_child(c)){
			item.set_child(c?)?;
		}
		Ok(item.finish()?)
	}
}