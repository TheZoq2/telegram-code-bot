use std::collections::HashMap;

#[derive(Serialize)]
pub struct GistFile {
    content: String
}

#[derive(Serialize)]
pub struct GistCreateRequest {
    public: bool,
    files: HashMap<String, GistFile>
}

impl GistCreateRequest {
    pub fn new_single_file(filename: String, content: String) -> Self{
        let mut files = HashMap::new();
        files.insert(filename, GistFile{content});
        Self {
            public: true,
            files
        }
    }
}
