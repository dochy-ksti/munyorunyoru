use crate::{builder::builder::Builder, error::parse_fail::ParseFail};

pub(crate) struct TreeItem<B> {
    pub item: B,
    pub children: Vec<TreeItem<B>>,
    pub start_index: usize,
}

impl<B> TreeItem<B> {
    pub(crate) fn new(item: B, start_index: usize) -> Self {
        Self {
            item,
            children: vec![],
            start_index,
        }
    }
}

pub(crate) struct BuilderTree<B> {
    pub root: Vec<TreeItem<B>>,
}

impl<B> BuilderTree<B> {
    pub(crate) fn new() -> Self {
        Self { root: vec![] }
    }
    pub(crate) fn add(&mut self, item: B, level: usize, start_index: usize) -> Result<(), String> {
        let mut parent = &mut self.root;
        for i in 0..level {
            let last = parent.last_mut();
            if last.is_none() {
                Err(format!(
                    "The item with indent level {level} is not found. The max level is {i}"
                ))?
            }
            parent = &mut last.unwrap().children;
        }
        parent.push(TreeItem::new(item, start_index));
        Ok(())
    }
}

impl<B, T> BuilderTree<B>
where
    B: Builder<Item = T>,
{
    pub(crate) fn finish(self) -> Result<Vec<T>, ParseFail> {
        self.root
            .into_iter()
            .map(|c| {
                Self::make_child(c)
                    .map(|(_, item)| item)
                    .map_err(|(index, m)| ParseFail::new(index, m))
            })
            .collect()
    }

    ///Returns Failed item's start index too
    fn make_child(tree_item: TreeItem<B>) -> Result<(usize, T), (usize, String)> {
        let children = tree_item.children;
        let mut item = tree_item.item;
        for c in children.into_iter().map(|c| Self::make_child(c)) {
            let (index, r) = c?;
            item.set_child(r).map_err(|e| (index, e))?; //use the child's index to report error
        }
        let r = item.finish().map_err(|e| (tree_item.start_index, e))?;
        Ok((tree_item.start_index, r))
    }
}
