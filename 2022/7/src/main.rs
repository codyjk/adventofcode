use std::fs::File;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut fs = FileSystemManager::new();
    let input = read_from_file("input.txt");
    let log_lines = parse_raw_log(&input);

    for line in log_lines {
        fs.handle_log_line(&line);
    }
}

fn part2() {
    let commands = read_from_file("input.txt");
}

fn read_from_file(input: &str) -> String {
    let mut file = File::open(input).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    contents
}

#[derive(Debug, Clone)]
enum LogLine {
    Command { command: Command },
    Stdout { value: String },
}

#[derive(Debug, Clone)]
enum Command {
    ChangeDir { target: String },
    ListDir,
}

fn parse_raw_log(raw_lines: &str) -> Vec<LogLine> {
    let mut log_lines = Vec::new();
    for curr_log_line in raw_lines.lines() {
        if curr_log_line.starts_with("$") {
            let full_command = curr_log_line[2..].to_string();
            let command_parts = full_command.split(" ").collect::<Vec<&str>>();
            let command = command_parts[0].to_string();
            let log_line = match command.as_str() {
                "cd" => LogLine::Command {
                    command: Command::ChangeDir {
                        target: command_parts[1].to_string(),
                    },
                },
                "ls" => LogLine::Command { command: Command::ListDir },
                _ => panic!("Unknown command: {}", command),
            };
            log_lines.push(log_line);
        } else {
            log_lines.push(LogLine::Stdout { value: curr_log_line.to_string() });
        }
    }
    log_lines
}

struct FileSystemManager<'a> {
    root: &'a DirectoryItem,
    visited_directory_stack: Vec<&'a DirectoryItem>,
    current_command: Option<Command>,
}

impl FileSystemManager<'_> {
    fn new() -> FileSystemManager<'static> {
        let root = DirectoryItem::Directory {
            name: "/".to_string(),
            items: Vec::new(),

        };
        let visited_directory_stack = vec![&root];
        FileSystemManager {
            root: &root,
            visited_directory_stack,
            current_command: None,
        }
    }

    fn peek_current_directory(&mut self) -> &mut DirectoryItem {
        self.visited_directory_stack.last_mut().unwrap()
    }

    fn push_directory(&mut self, directory: &DirectoryItem) {
        self.visited_directory_stack.push(directory);
    }

    fn go_back_to_last_directory(&mut self) {
        self.visited_directory_stack.pop();
    }

    fn handle_log_line(&mut self, log_line: &LogLine) {
        match log_line {
            LogLine::Command { command } => {
                self.current_command = Some(command.clone());
                self.execute_command(command);
            }
            LogLine::Stdout { value } => {
                match self.current_command.clone() {
                    Some(Command::ChangeDir { target: _ }) => {
                        panic!("Should not get Stdout after changing dir");
                    }
                    Some(Command::ListDir) => {
                        let parts = value.split(" ").collect::<Vec<&str>>();
                        let is_dir = parts[0] == "dir";
                        let name = parts[1].to_string();

                        let item = if is_dir {
                            DirectoryItem::Directory {
                                name,
                                items: Vec::new(),
                            }
                        } else {
                            DirectoryItem::File {
                                name,
                                size: parts[0].parse::<u32>().unwrap(),
                            }
                        };

                        match self.peek_current_directory() {
                            DirectoryItem::Directory { items, .. } => {
                                items.push(item);
                            }
                            DirectoryItem::File { .. } => {
                                panic!("Current dir is a file");
                            }
                        }
                    }
                    None => panic!("No current command!"),
                }
            }
        }
    }

    fn execute_command(&mut self, command: &Command) {
        match command {
            Command::ChangeDir { target } => {
                match self.peek_current_directory() {
                    DirectoryItem::Directory { items, .. } => {
                        let mut found = false;
                        for item in items {
                            match item {
                                DirectoryItem::Directory { name, .. } => {
                                    if name == target {
                                        self.push_directory(&item);
                                        found = true;
                                        break;
                                    }
                                }
                                DirectoryItem::File { .. } => {}
                            }
                        }
                        if !found {
                            let new_dir = DirectoryItem::Directory {
                                name: target.to_string(),
                                items: Vec::new(),
                            };
                            items.push(new_dir);
                            self.push_directory(items.last().unwrap());
                        }
                    }
                    DirectoryItem::File { .. } => {
                        panic!("Current dir is a file");
                    }
                }
            }
            Command::ListDir => {}
        }
    }
}

enum DirectoryItem {
    Directory { name: String, items: Vec<DirectoryItem> },
    File { name: String, size: u32 },
}
