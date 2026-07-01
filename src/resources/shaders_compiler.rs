use std::collections::{HashMap, HashSet};
use std::path::{Path};


const SHADER_HEADER_EXTENSION: &str = "glsl";

pub struct ShadersCompiler {
    shader_folder: String,

    includes_cache: HashMap<String, String>,
    processed_includes: HashSet<String>,

    compiler: Option<shaderc::Compiler>,
}

impl ShadersCompiler {
    pub fn new() -> Self {
        Self {
            shader_folder: String::new(),

            includes_cache: HashMap::new(),
            processed_includes: HashSet::new(),

            compiler: None,
        }
    }

    pub fn start(&mut self, shader_folder: String) {
        self.shader_folder = shader_folder;
        self.compiler = Some(shaderc::Compiler::new().unwrap());

        let cache_dir = format!("{}/cache", self.shader_folder);

        if !std::fs::exists(&cache_dir).unwrap_or(false) {
            std::fs::create_dir(&cache_dir).expect("Failed to create shaders cache directory");
        }
    }

    pub fn compile(&mut self, shader_name: &str, kind: shaderc::ShaderKind) -> Vec<u8> {
        let mut full_path = format!("{}/{shader_name}", self.shader_folder);
        full_path = full_path.replace('\\', "/");

        // try read shader cache
        if let Some(cache) = self.read_compiled_cache(shader_name, &full_path) {
            return cache;
        }

        let file_name = Path::new(&full_path).file_name().unwrap().to_str().unwrap();

        let mut source_text = std::fs::read_to_string(&full_path).expect("Error to read shader source");

        self.processed_includes.clear();


        // get a slice without file name and file extension
        let parent_dir = &full_path[0..full_path.len() - (full_path.len() - full_path.rfind('/').unwrap())];

        self.process_include_directive(&parent_dir, &mut source_text);

        match self.compiler.as_ref().unwrap().compile_into_spirv(&source_text, kind, file_name, "main", None) {
            Ok(result) => {
                let binary_code = result.as_binary_u8();
                self.update_cache(shader_name, &full_path, &binary_code);

                return binary_code.to_vec();
            }
            Err(err) => panic!("{err}")
        }
    }

    fn update_cache(&self, shader_name: &str, source_path: &str, content: &[u8]) {
        let full_path = format!("{}/cache/{shader_name}", self.shader_folder);
        let dir_path = Path::new(&full_path).parent().unwrap();

        // create cache file parent directory
        if !Path::is_dir(dir_path) {
            std::fs::create_dir(dir_path).expect("Failed to create shaders cache directory!");
        }

        // write cache in file
        std::fs::write(&full_path, content).expect("Error to write cache file");

        // change cache modified time metada by the source modified time
        let source_metadata = std::fs::metadata(full_path).unwrap();
        let modified_metadata = filetime::FileTime::from_last_modification_time(&source_metadata);

        filetime::set_file_times(&source_path, filetime::FileTime::now(), modified_metadata)
            .expect("Error to set cache file modified time metadata");
    }

    fn read_compiled_cache(&self, shader_name: &str, full_path: &str) -> Option<Vec<u8>> {
        let cache_path = format!("{}/cache/{shader_name}", self.shader_folder);

        if std::fs::exists(&cache_path).unwrap_or(false) {
            let cache_metadata = std::fs::metadata(&cache_path).unwrap();
            let shader_metadata = std::fs::metadata(full_path).unwrap();

            // if modified time of both is different, then cache is outdated
            if shader_metadata.modified().unwrap() != cache_metadata.modified().unwrap() {
                return None;
            }

            return Some(std::fs::read(&cache_path).unwrap());
        }

        return None;
    }

    fn process_include_directive(&mut self, parent_dir: &str, source_text: &mut String) {
        let (mut file_name, start, end) = Self::process_logic(source_text, parent_dir, false);

        // that shader file do not have #include directives
        if file_name.is_empty() { return }

        // we can includes only header files
        if !file_name.ends_with(SHADER_HEADER_EXTENSION) {
            panic!("Can not includes files that arent headers")
        }

        // read include file
        let mut include_file_path;

        if Path::new(&file_name).is_absolute() {
            include_file_path = file_name.clone();
        }
        else {
            let mut p = parent_dir.to_string();

            Self::process_include_file_path(&mut p, &mut file_name);
            include_file_path = p;
        }

        // remove #include directive from the file
        source_text.replace_range(start..=end, "");

        // avoids include a file that was included
        if self.processed_includes.contains(&include_file_path) { return }
        self.processed_includes.insert(include_file_path.clone());


        // check if file is in cache or read orthewise
        let mut content = self.includes_cache.entry(include_file_path.clone()).or_insert_with(|| {
            std::fs::read_to_string(&include_file_path).expect("error to read include file")
        });

        // remove file name and file extension
        include_file_path.replace_range(include_file_path.rfind('/').unwrap().., "");

        // replace all relatives paths in file by absolutes paths
        Self::process_logic(&mut content, &include_file_path, true);

        // insert include file content in source file
        source_text.insert_str(start, content);


        // process all includes recursively
        self.process_include_directive(parent_dir, source_text);
    }

    fn process_logic(source_text: &mut String, parent_dir: &str, replace_relatives: bool) -> (String, usize, usize) {
        let mut file_name = String::new();
        let mut include_line_start = 0;
        let mut include_line_end = 0;

        let mut line_content = String::new();
        let mut read_line = false;
        let mut read_include_file_name = false;
        let mut slash_count = 0;
        let mut string_terminator_count = 0;
        let mut string_start = 0;

        for (i, c) in source_text.chars().enumerate() {
            // breakline
            if c == '\n' || c == '\r' || c == '\0' {
                line_content.clear();
                read_line = false;
                slash_count = 0;
                string_start = 0;
            }

            // if two '/' is together, then line is commented
            if c == '/' && !read_include_file_name { slash_count += 1 }

            // line is commented, then continue
            if slash_count == 2 {
                continue;
            }

            // reads just lines that have '#' to improve performance
            if c == '#' {
                read_line = true;
                include_line_start = i;
            }

            if read_line {
                line_content.push(c);
            }


            // check if line content is a #include directive and disable read line to avoids fake falses
            if line_content == "#include" {
                read_include_file_name = true;
                read_line = false;
            }

            // reads just content that was inside strings terminators
            if read_include_file_name {
                if c == '"' {
                    if string_start == 0 { string_start = i; }

                    string_terminator_count += 1;
                }
                else if string_terminator_count == 1 {
                    file_name.push(c);
                }
            }

            // string is terminated then return his content
            if string_terminator_count == 2 {
                include_line_end = i;

                if file_name.is_empty() {
                    panic!("Invalid include!")
                }

                if replace_relatives && !Path::new(&file_name).is_absolute() {
                    let mut absolute_path = parent_dir.to_string();
                    Self::process_include_file_path(&mut absolute_path, &mut file_name);

                    // remove content inside: ".." -> ""
                    source_text.replace_range(string_start + 1..include_line_end, "");

                    // insert content inside: "" -> ".."
                    source_text.insert_str(string_start + 1, &absolute_path);
                }

                break;
            }
        }

        // replace all relatives includes recursively
        if replace_relatives {
            if file_name.is_empty() || Path::new(&file_name).is_absolute() {
                return (file_name, include_line_start, include_line_end)
            }

            return Self::process_logic(source_text, parent_dir, true);
        }

        return (file_name, include_line_start, include_line_end);
    }

    fn process_include_file_path(path: &mut String, file_name: &mut String) {
        if file_name.starts_with("../") {
            file_name.replace_range(0..3, "");

            path.replace_range(path.rfind('/').unwrap().., "");

            Self::process_include_file_path(path, file_name);
        }
        else {
            path.push('/');
            path.push_str(file_name);
        }
    }
}
