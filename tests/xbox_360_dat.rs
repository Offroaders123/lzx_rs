use std::path::PathBuf;

pub trait ConsoleParser {
    // fn discover_save_layout(&self, root_folder: &PathBuf) -> SaveLayout;
    fn inflate_from_layout(&mut self, the_save: &SaveProject, in_file_path: &PathBuf) -> Status;

    // fn deflate_to_save(&self, save_project: &SaveProject, the_settings: &WriteSettings) -> i32;
    // fn supply_required_defaults(&self, save_project: &SaveProject) -> ();

    // protected:

    fn inflate_listing(&self, save_project: &SaveProject) -> Status;
    // fn deflate_listing(
    //     &self,
    //     game_data_path: &PathBuf,
    //     inflated_data: &Buffer,
    //     deflated_data: &Buffer,
    // ) -> i32;

    // fn read_file_info(&self, save_project: &SaveProject) -> ();
}

pub struct SaveLayout;
pub struct SaveProject;
pub struct WriteSettings;
pub struct Buffer;

pub struct Xbox360Dat {
    m_console: Console,
    m_file_path: Option<PathBuf>,
}

pub enum Console {
    Xbox360,
}

impl Xbox360Dat {
    pub fn new() -> Self {
        Xbox360Dat {
            m_console: Console::Xbox360,
            m_file_path: None,
        }
    }
}

impl ConsoleParser for Xbox360Dat {
    fn inflate_from_layout(&mut self, save_project: &SaveProject, the_file_path: &PathBuf) -> Status {
        self.m_file_path = Some(the_file_path.clone());

        let status: Status = self.inflate_listing(save_project);
        match status {
            Status::Compress => (),
            _ => {
                println!("failed to extract listing\n");
                return status;
            }
        }

        // readFileInfo(save_project);

        return Status::Success;
    }

    fn inflate_listing(&self, save_project: &SaveProject) -> Status {
        todo!()
    }
}

pub enum Status {
    Success = 0,
    Compress = -1,
    Decompress = -2,
    MallocFailed = -3,
    InvalidSave = -4,
    FileError = -5,
    InvalidConsole = -6,
    InvalidArgument = -7,
    NotImplemented = -8,
}
