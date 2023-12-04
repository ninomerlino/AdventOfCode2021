use std::{
    cell::RefCell,
    default,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::common::{Result, SillyStringError};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum VisibilityState {
    #[default]
    Uknown,
    Visible,
    Invisible(u8),
}

impl From<VisibilityState> for bool {
    fn from(value: VisibilityState) -> Self {
        value == VisibilityState::Visible
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Visibility {
    top: VisibilityState,
    bottom: VisibilityState,
    left: VisibilityState,
    right: VisibilityState,
}

impl From<Visibility> for bool {
    fn from(value: Visibility) -> Self {
        value.bottom.into() || value.top.into() || value.right.into() || value.left.into()
    }
}

#[derive(Debug, Default, Clone)]
struct Map {
    tree_map: Vec<u8>,
    visible_trees: RefCell<Vec<Visibility>>,
    map_width: usize,
    map_height: usize,
}

impl Map {
    fn new(file: &File) -> Result<Self> {
        let mut stream = BufReader::new(file);
        let mut tree_map: Vec<u8> = Vec::new();
        let mut map_width = 0;
        let mut map_height = 0;
        for line in stream.lines() {
            let line = line?;
            let bytes = line.as_bytes();
            if map_width == 0 {
                map_width = bytes.len();
            } else if bytes.len() != map_width {
                Err(SillyStringError::new(
                    &"Line has different size from the others",
                ))?
            }
            tree_map.extend(bytes);
            map_height += 1;
        }
        let visible_trees = RefCell::new(vec![Visibility::default(); tree_map.len()]);
        for i in 0..map_width{
            
        }
        Ok(Self {
            tree_map,
            visible_trees,
            map_width,
            map_height,
        })
    }

    #[inline]
    pub fn to_index(&self, x: usize, y: usize) -> usize {
        x + self.map_width * y
    }
    #[inline]
    pub fn check_top_visibility(&self, x: usize, y: usize) -> VisibilityState {
        match self.visible_trees.borrow_mut()[self.to_index(x, y)].top {
            VisibilityState::Uknown => {
                
            }
            state => state,
        }
    }

    pub fn is_tree_visible(&self, x: usize, y: usize) -> bool {}
}
