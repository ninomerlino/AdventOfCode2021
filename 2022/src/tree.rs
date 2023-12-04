use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    ChildIndexOutOfBound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Debug).fmt(f)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Node<T: Debug> {
    parent: Option<*mut Node<T>>,
    value: T,
    children: Vec<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Debug,
{
    fn new(parent: Option<*mut Self>, value: T) -> Box<Self> {
        Box::new(Self {
            parent,
            value,
            children: Vec::default(),
        })
    }
    fn get_self_ptr(&mut self) -> *mut Self {
        self
    }
    fn add_child(&mut self, value: T) {
        let child_node = Node::new(Some(self.get_self_ptr()), value);
        self.children.push(child_node);
    }
    fn get_value(&self) -> &T {
        &self.value
    }
    fn get_child(&mut self, index: usize) -> Result<&mut Self> {
        if index >= self.children.len() {
            return Err(Error::ChildIndexOutOfBound);
        }
        Ok(self.children[index].as_mut())
    }
    fn len(&self) -> usize {
        self.children.len()
    }
    fn get_children(&self) -> Vec<&T> {
        self.children.iter().map(|node| node.get_value()).collect()
    }
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl<T> Node<T>
where
    T: Debug + ToString,
{
    fn to_display_string(&self, depth: usize, last_one: bool) -> String {
        let mut display_string = String::new();
        let mut children_padding = String::new();
        if depth > 0 {
            for _ in 0..depth - 1 {
                children_padding.push('┃');
            }
            children_padding.push(if last_one { ' ' } else { '┃' });
        }
        if depth == 0 {
            display_string.push('●');
        } else {
            display_string.push(if last_one { '┗' } else { '┣' });
        }
        display_string += &self.value.to_string();
        display_string.push('\n');
        if !self.is_leaf() {
            let last_index = self.children.len() - 1;
            for i in 0..=last_index {
                let child = &self.children[i];
                display_string += &children_padding;
                display_string += &child.to_display_string(depth + 1, i == last_index);
            }
        }
        display_string
    }
    unsafe fn get_path(&self) -> String {
        match self.parent {
            Some(parent) => (*parent).get_path() + &self.value.to_string(),
            None => self.value.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Tree<T: Debug> {
    root: Box<Node<T>>,
    current: *mut Node<T>,
}

impl<T> Tree<T>
where
    T: Debug,
{
    pub fn new(value: T) -> Self {
        let mut root = Node::new(None, value);
        let current = root.get_self_ptr();
        Self { root, current }
    }
    pub fn get_current(&self) -> &T {
        unsafe { (*self.current).get_value() }
    }
    pub fn go_to_parent(&mut self) -> Option<&T> {
        unsafe {
            match (*self.current).parent {
                Some(value) => {
                    self.current = value;
                }
                None => return None,
            };
        };
        Some(self.get_current())
    }
    pub fn go_to_child(&mut self, index: usize) -> Result<&T> {
        unsafe {
            self.current = (*self.current).get_child(index)? as *mut Node<T>;
        }
        Ok(self.get_current())
    }
    pub fn add_child(&mut self, value: T) {
        unsafe { (*self.current).add_child(value) }
    }
    pub fn children_count(&self) -> usize {
        unsafe { (*self.current).len() }
    }
    pub fn return_to_root(&mut self) {
        self.current = self.root.get_self_ptr();
    }
}

impl<T> Tree<T>
where
    T: Debug + ToString,
{
    pub fn pwd(&self) -> String {
        unsafe { (*self.current).get_path() }
    }
}

impl<T> Tree<T>
where
    T: Debug + Clone,
{
    pub fn get_children(&self) -> Vec<&T> {
        unsafe { (*self.current).get_children() }
    }
}

impl<T> Display for Tree<T>
where
    T: Debug + ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.write_str(&(*self.current).to_display_string(0, false))?;
        }
        Ok(())
    }
}
