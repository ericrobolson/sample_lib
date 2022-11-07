use super::get_files;
use std::{io::Write, path::PathBuf};

/// A node in a folder heirarchy.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    path: PathBuf,
    title: String,
    description: String,
    children: Vec<Self>,
}

impl Node {
    /// Creates a new empty node.
    pub fn new<'a>(title: &'a str, description: &'a str) -> Self {
        Self {
            path: title.clone().into(),
            title: title.into(),
            description: description.into(),
            children: vec![],
        }
    }

    /// Returns whether the node is empty or not.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Loads the given heirarchy from disk.
    pub fn load<'a>(path: &'a PathBuf) -> Self {
        let title = path.file_name().unwrap().to_str().unwrap().to_string();

        let mut node = Self {
            path: path.clone(),
            title,
            description: String::new(),
            children: vec![],
        };

        node.description = {
            let path = node.description_path(path);
            match std::fs::read_to_string(path) {
                Ok(description) => description.trim().to_string(),
                Err(_) => "Empty description".into(),
            }
        };

        for dir in get_files(path)
            .iter()
            .filter_map(|f| if f.path().is_dir() { Some(f) } else { None })
        {
            let path = dir.path();
            node.children.push(Self::load(&path))
        }

        node.sort_children();
        node.rebuild_path(None);
        node
    }

    /// Pushes the given node onto the heirarchy.
    pub fn push(&mut self, node: Node) -> &mut Self {
        self.children.push(node);
        self
    }

    /// Saves the heirarchy to disk.
    pub fn save(&self) {
        let path = self.path.clone();
        // Ensure path exists
        if !path.exists() {
            std::fs::create_dir_all(&path).unwrap();
        }

        // Save description
        {
            let mut f = std::fs::File::create(self.description_path(&path)).unwrap();
            f.write(self.description.as_bytes()).unwrap();
        }

        // Save children
        for child in self.children.iter() {
            child.save();
        }
    }

    /// Sorts the children by title.
    pub fn sort_children(&mut self) {
        self.children.sort_by(|a, b| a.title.cmp(&b.title))
    }

    /// The filepath for the description file.
    fn description_path<'a>(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.clone();
        path.push(format!("{}_description.txt", self.title));
        path
    }

    /// Rebuilds the path to be logically consistent.
    pub fn rebuild_path(&mut self, path: Option<PathBuf>) {
        self.path = match path {
            Some(mut p) => {
                p.push(&self.title);
                p
            }
            None => self.path.clone(),
        };

        for child in self.children.iter_mut() {
            child.rebuild_path(Some(self.path.clone()));
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Node {
    fn from((title, description): (&'a str, &'a str)) -> Self {
        Self::new(title, description)
    }
}
