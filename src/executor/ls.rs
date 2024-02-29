use std::{
    env,
    fmt::Debug,
    fs,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::PathBuf,
    rc::Rc,
};

use colored::{ColoredString, Colorize};

use crate::parser::ast_node_trait::CommandAstNode;
use crate::{
    executor::Command,
    stream::{self, Stream},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FileType {
    File,
    Dir,
    Link,
    CharDevice,
    BlockDevice,
    Fifo,
    Socket,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FileInfo {
    file_type: FileType,
    permissions: String,
    link: u64,
    owner: String,
    group: String,
    size: u64,
    modified_time: String,
    name: String,
    is_hidden: bool,
}

// Ls command
pub struct LsCmd {
    // show details of files and directories
    long: bool,

    // show hidden files and directories
    all: bool,

    // show human readable file sizes
    human_readable: bool,

    // sort by file size
    sort_by_size: bool,

    // sort by modified time
    sort_by_time: bool,

    // reverse sort
    reverse: bool,

    // show files and directories as a tree
    tree: bool,

    // set the depth of the tree, default is 10
    depth: u8,

    // set file or directory path
    paths: Vec<PathBuf>,

    // This field instruct the program what to do.
    // 'ls'                     => status-0 : default status
    // 'ls -l'                  => status-1 : show details of files and directories
    // 'ls -a'                  => status-2 : show hidden files and directories
    // 'ls -a -l'               => status-3 : calculated by 1 | 2, it will show details of all hidden files and directories
    // 'ls -h'                  => status-4 : set status to 4, but do nothing, don't ask why, Linux ls command also do nothing when get '-h' option
    // 'ls -l -h'               => status-5 : calculated by 1 | 4, it will show details of files and directories with human readable file sizes
    // 'ls -a -l -h'            => status-7 : calculated by 1 | 2 | 4, it will show details of all hidden files and directories with human readable file sizes
    // 'ls -t' of 'ls --tree'   => status-8 : show files and directories as a tree
    // other command            => status-0 : default status
    // Above status were set by the parse function what we implemented in the impl code block.
    //
    // Attention: You must use #[arg(skip)] to skip the hidden field,
    // otherwise it will be shown in help message, and even panic will appear in the program!!!
    status: u8,

    stream: Option<Rc<dyn Stream>>,
}

impl LsCmd {
    fn new() -> Self {
        Self {
            long: false,
            all: false,
            human_readable: false,
            sort_by_size: false,
            sort_by_time: false,
            reverse: false,
            tree: false,
            depth: 10,
            paths: Vec::new(),
            status: 0,
            stream: None,
        }
    }

    // Set status of the command
    fn set_status(&mut self) {
        // Set status to 0 by default
        self.status = 0;

        // Set status to 1 if get '-l' option
        if self.long {
            self.status |= 1;
        }

        // Set status to 2 if get '-a' option
        if self.all {
            self.status |= 2;
        }

        // Set status to 4 if get '-H' option
        if self.human_readable {
            self.status |= 4;
        }

        if self.tree {
            self.status |= 8;
        }
    }

    // If don't get any option or use other options that don't define,
    // just show non-hidden files name.
    fn show_names(&self, files: &Vec<FileInfo>) {
        let mut files_str = String::new();

        for file in files.iter() {
            if !self.all && file.is_hidden {
                continue;
            }

            files_str.push_str(format!("{:<20}", self.color_file_names(&file)).as_str());
        }

        self.stream
            .as_ref()
            .unwrap()
            .input(files_str);
    }

    // Show details of files and directories
    fn show_infos(&self, files: &Vec<FileInfo>) {
        for file in files.iter() {
            if !self.all && file.is_hidden {
                continue;
            }

            let size = if self.human_readable {
                self.human_readable_size(file.size)
            } else {
                file.size.to_string()
            };

            let file_name_with_color = self.color_file_names(&file);

            self.stream.as_ref().unwrap().input(format!(
                "{} {:>3} {:>8} {:>8} {:>8} {:>20} {}",
                file.permissions,
                file.link,
                file.owner,
                file.group,
                size,
                file.modified_time,
                file_name_with_color
            ));
        }
    }

    // Show files and directories as a tree.
    fn show_as_tree(&self, path: &std::path::PathBuf) {
        self.show_as_tree_recursively(path, 0);
    }

    // Show files and directories as a tree recursively.
    #[cfg(unix)]
    fn show_as_tree_recursively(&self, path: &std::path::PathBuf, depth: u8) {
        if !path.exists() {
            self.stream.as_ref().unwrap().input(format!(
                "{:indent$}| - {}",
                "",
                "No such file or directory".red(),
                indent = (depth * 5) as usize
            ));
            return;
        }

        if depth > self.depth {
            return;
        }

        // Get file info.
        let file_info = self.get_file_info(path);

        // Get file name with color.
        let file_name_with_color = self.color_file_names(&file_info);

        // Print file name with color.
        self.stream.as_ref().unwrap().input(format!(
            "{:indent$}| - {}",
            "",
            file_name_with_color,
            indent = (depth * 5) as usize
        ));

        // If the file is a directory, get all files and directories in it.
        if file_info.file_type == FileType::Dir {
            let paths = match fs::read_dir(path) {
                Ok(paths) => paths,
                Err(_) => {
                    self.stream.as_ref().unwrap().input(format!(
                        "{:indent$}| - {}",
                        "",
                        "Permission denied".red(),
                        indent = (depth * 5) as usize
                    ));
                    return;
                }
            };
            for path in paths {
                let path = path.unwrap().path();
                self.show_as_tree_recursively(&path, depth + 1);
            }
        }
    }

    // Color file name by file type when show file names.
    fn color_file_names(&self, file: &FileInfo) -> ColoredString {
        match file.file_type {
            FileType::File => file.name.white(),
            FileType::Dir => file.name.cyan(),
            FileType::Link => file.name.blue(),
            FileType::CharDevice | FileType::BlockDevice | FileType::Fifo | FileType::Socket => {
                file.name.green()
            }
        }
    }

    // Turn file size to human-readable size.
    fn human_readable_size(&self, size: u64) -> String {
        let mut size = size as f64;
        let mut unit = "B";

        if size > 1024.0 {
            size /= 1024.0;
            unit = "K";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "M";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "G";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "T";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "P";
        }

        format!("{:.2}{}", size, unit)
    }

    // Get file info, such as file size, modified time, etc.
    #[cfg(unix)]
    fn get_file_info(&self, path_buf: &std::path::PathBuf) -> FileInfo {
        // Get file metadata, include file size, modified time, etc.
        use chrono::{DateTime, Local};
        let metadata = match fs::symlink_metadata(path_buf) {
            Ok(metadata) => metadata,
            Err(_) => path_buf.metadata().unwrap(),
        };

        // Get file basic info include: permissions, type, name and is not hidden.
        let (permission, file_type) = self.analysis_mode(&metadata);

        // Get file name and judge if it is hidden.
        let file_name = match path_buf.file_name() {
            Some(file_name) => {
                let file_name = file_name.to_string_lossy().into_owned();
                file_name
            }
            None => {
                panic!("{}", format!("Error: Can't get file name").red());
            }
        };
        let is_hidden: bool = file_name.starts_with(".");

        // Get file link number.
        let link_num = metadata.nlink();

        // Get modified time of file.
        let modify_time: DateTime<Local> = metadata.modified().unwrap().into();
        let modify_time = modify_time.format("%Y-%m-%d %H:%M:%S").to_string();

        // Get owner and group name.
        let (owner_name, group_name) = self.get_owner_and_group_name(&metadata, &file_type);

        // Store these infos to FileInfo struct and add it to vec.
        let fi = FileInfo {
            permissions: permission,
            file_type: file_type,
            link: link_num,
            owner: owner_name,
            group: group_name,
            size: metadata.len(),
            modified_time: modify_time,
            name: file_name,
            is_hidden,
        };

        fi
    }

    // Get owner and group name.
    #[cfg(unix)]
    fn get_owner_and_group_name(
        &self,
        metadata: &fs::Metadata,
        file_type: &FileType,
    ) -> (String, String) {
        use std::ffi::CStr;

        use libc::getgrgid;
        use users::{get_group_by_gid, get_user_by_uid};

        let group_name: String;

        let uid = metadata.uid();
        let gid = metadata.gid();

        // If the file type is not file, dir or link, just one way to get group name by libc.
        // It's so difficult to get group name by std::os::unix::fs::MetadataExt and users crate.
        // Because The method in the 'user crate' for converting a gid to a group name
        // can cause the program to panic due to memory alignment issues.
        // So it is necessary to use libc to call the C language implementation to accomplish this functionality.
        if file_type != &FileType::File
            || file_type != &FileType::Dir
            || file_type != &FileType::Link
        {
            // 获取用户组名
            let group_info = unsafe { getgrgid(gid) };
            group_name = if !group_info.is_null() {
                let group_name_cstr = unsafe { CStr::from_ptr((*group_info).gr_name) };
                group_name_cstr.to_string_lossy().into_owned()
            } else {
                "".to_string()
            }
        } else {
            group_name = get_group_by_gid(gid)
                .map(|g| g.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| "Unknown".to_string());
        }

        let owner_name = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| "Unknown".to_string());

        return (owner_name, group_name);
    }

    #[cfg(unix)]
    fn analysis_mode(&self, metadata: &fs::Metadata) -> (String, FileType) {
        // Get file permissions.
        let mode: u32 = metadata.permissions().mode();

        // Turn permission number to string.
        let perms_str = format!(
            "{}{}{}",
            self.turn_permission_num_to_str((mode >> 6) & 0o007),
            self.turn_permission_num_to_str((mode >> 3) & 0o007),
            self.turn_permission_num_to_str(mode & 0o007)
        );

        // Get file type, and add it to the msg.
        let file_type = metadata.file_type();
        let result = match file_type {
            _ if file_type.is_dir() => (format!("d{perms_str}"), FileType::Dir),
            _ if file_type.is_file() => (format!("-{perms_str}"), FileType::File),
            _ if file_type.is_symlink() => (format!("l{perms_str}"), FileType::Link),
            _ if file_type.is_char_device() => (format!("c{perms_str}"), FileType::CharDevice),
            _ if file_type.is_block_device() => (format!("b{perms_str}"), FileType::BlockDevice),
            _ if file_type.is_fifo() => (format!("p{perms_str}"), FileType::Fifo),
            _ if file_type.is_socket() => (format!("s{perms_str}"), FileType::Socket),
            _ => (format!("?{perms_str}"), FileType::File),
        };

        result
    }

    // Turn permission number to string.
    // For example: 0o755 => rwxr-xr-x
    #[cfg(unix)]
    fn turn_permission_num_to_str(&self, num: u32) -> String {
        let mut result = String::from("");

        if num & 4 == 4 {
            result.push_str("r");
        } else {
            result.push_str("-");
        }

        if num & 2 == 2 {
            result.push_str("w");
        } else {
            result.push_str("-");
        }

        if num & 1 == 1 {
            result.push_str("x");
        } else {
            result.push_str("-");
        }

        result
    }

    // Just print files and dirs name in the path
    #[cfg(unix)]
    fn get_files_and_dirs(&self, path_buf: &PathBuf, files: &mut Vec<FileInfo>) {
        // Check if the path is a file.
        if !path_buf.is_dir() {
            // If it is a file, just get file info and return.
            files.push(self.get_file_info(path_buf));
            return;
        } else {
            // If it is a directory, get all files and directories in it.
            // And store them to the vec.
            let paths = match fs::read_dir(path_buf) {
                Ok(paths) => paths,
                Err(_) => {
                    let msg = format!("Error: Permission denied").red();
                    panic!("{}", msg);
                }
            };
            for path in paths {
                let path = path.unwrap().path();
                files.push(self.get_file_info(&path));
            }
        }

        // Sort by option
        if self.sort_by_size {
            files.sort_by(|f1, f2| f1.size.cmp(&f2.size));
        } else if self.sort_by_time {
            files.sort_by(|f1, f2: &FileInfo| f1.modified_time.cmp(&f2.modified_time));
        } else {
            files.sort_by(|f1, f2| f1.name.cmp(&f2.name));
        }

        // Reverse sort if get '-r' option.
        if self.reverse {
            files.reverse();
        }
    }
}

impl From<Box<dyn CommandAstNode>> for LsCmd {
    fn from(cmd: Box<dyn CommandAstNode>) -> Self {
        let mut ls_cmd = Self::new();

        // Get the paths from 'cmd.values'
        ls_cmd.paths = match cmd.get_values() {
            Some(values) => values.into_iter().map(PathBuf::from).collect(),
            None => Vec::new(),
        };
        // Set paths default value
        if ls_cmd.paths.is_empty() {
            match env::current_dir() {
                Ok(path) => {
                    ls_cmd.paths.push(path);
                }
                Err(e) => {
                    println!("Failed to get current directory: {}", e);
                }
            }
        }

        // Get the 'long' option
        match cmd.get_option("-l").or(cmd.get_option("--long")) {
            Some(_) => ls_cmd.long = true,
            None => ls_cmd.long = false,
        }

        // Get the 'all' option
        match cmd.get_option("-a").or(cmd.get_option("--all")) {
            Some(_) => ls_cmd.all = true,
            None => ls_cmd.all = false,
        }

        // Get the 'human_readable' option
        match cmd.get_option("-h").or(cmd.get_option("--human-readable")) {
            Some(_) => ls_cmd.human_readable = true,
            None => ls_cmd.human_readable = false,
        }

        // Get the 'resort' option
        match cmd.get_option("-r").or(cmd.get_option("--reverse")) {
            Some(_) => ls_cmd.reverse = true,
            None => ls_cmd.reverse = false,
        }

        match cmd.get_option("-t").or(cmd.get_option("--time")) {
            Some(_) => ls_cmd.sort_by_time = true,
            None => ls_cmd.sort_by_time = false,
        }

        match cmd.get_option("-s").or(cmd.get_option("--size")) {
            Some(_) => ls_cmd.sort_by_size = true,
            None => ls_cmd.sort_by_size = false,
        }

        // Get the 'tree' option
        match cmd.get_option("--tree") {
            Some(_) => ls_cmd.tree = true,
            None => ls_cmd.tree = false,
        }

        // Get the 'depth' option
        match cmd.get_option("--depth") {
            Some(depth) => ls_cmd.depth = depth.parse::<u8>().unwrap_or(3),
            None => ls_cmd.depth = 3,
        }

        // Initialize the status
        ls_cmd.set_status();

        ls_cmd
    }
}

impl Command for LsCmd {
    fn execute(&self) {
        self.paths.iter().for_each(|path| {
            let mut files = Vec::new();
            self.get_files_and_dirs(path, &mut files);

            let _v = match self.status {
                0 | 2 | 4 => self.show_names(&files),
                1 | 3 | 5 | 7 => self.show_infos(&files),
                8 => self.show_as_tree(path),
                _ => self.show_names(&files),
            };
        });
    }

    fn add_stream(&mut self, stream: Rc<dyn stream::Stream>) {
        self.stream = Some(stream);
    }
}
