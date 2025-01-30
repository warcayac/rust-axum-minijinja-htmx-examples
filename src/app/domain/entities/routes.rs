use serde::Deserialize;


#[derive(Clone, Deserialize)]
pub struct Routes {
    pub alias: String,
    pub base: String,
    pub paths: Vec<Path>,
}

impl Routes {
    pub fn get_alias(&self) -> String {
        self.alias.clone()
    }

    pub fn get_base(&self) -> String {
        self.base.clone()
    }

    pub fn get_path(&self, index: usize) -> Path {
        self.paths.get(index).unwrap().clone()
    }

    pub fn pathname(&self, index: usize) -> String {
        self.paths.get(index).unwrap().name.clone()
    }

    pub fn pathname_variant(&self, path_idx: usize, opt_idx: usize) -> String {
        self.paths.get(path_idx).unwrap().variant(opt_idx)
    }

    pub fn full_pathname(&self, index: usize) -> String {
        format!("{}{}", self.base, self.pathname(index))
    }
    
    pub fn full_pathname_variant(&self, path_idx: usize, opt_idx: usize) -> String {
        format!("{}{}", self.base, self.pathname_variant(path_idx, opt_idx))
    }

    // The '_ lifetime specifier indicates the iterator borrows from self
    pub fn full_pathnames(&self) -> impl Iterator<Item = String> + '_ {
        self.paths
            .iter()
            .enumerate()
            .map(|(i, _)| self.full_pathname(i))
    }
}

impl Iterator for Routes {
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        self.paths.pop()
    }
}

#[derive(Clone, Deserialize)]
pub struct Path {
    pub name: String,
    pub options: Vec<String>,
}

impl Path {
    pub fn variant(&self, index: usize) -> String {
        format!("{}/{}", self.name, self.options[index])
    }
}