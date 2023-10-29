use crate::builder::builder::Builder;

pub(crate) struct TreeItem<B> {
    pub item: B,
    pub children: Vec<TreeItem<B>>,
}

impl<B> TreeItem<B> {
    pub(crate) fn new(item: B) -> Self {
        Self {
            item,
            children: vec![],
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
    pub(crate) fn new(root: B) -> Self {
        Self {
            root: TreeItem::new(root),
        }
    }
    pub(crate) fn add(&mut self, item: B, level: usize) -> Result<(), String> {
        let mut parent = Some(&mut self.root);
        for i in 0..level {
            parent = parent.unwrap().last_mut();
            if parent.is_none() {
                Err(format!(
                    "The item with indent level {level} is not found. The max level is {i}"
                ))?
            }
        }
        parent.unwrap().children.push(TreeItem::new(item));
        Ok(())
    }
}

impl<B,T> BuilderTree<B> where B : Builder<Item=T>{
	pub(crate) fn finish(self) -> Vec<T>{
		self.root.children.into_iter().map(|c| Self::make_child(c)).collect()
	}

	fn make_child(item : TreeItem<B>) -> T{
		let children = item.children;
		let mut item = item.item;
		for c in children.into_iter().map(|c| Self::make_child(c)){
			item.set_child(c);
		}
		item.finish()
	}
}