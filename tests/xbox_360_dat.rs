use std::path::PathBuf;

pub trait ConsoleParser {
    // fn discover_save_layout(&self, root_folder: &PathBuf) -> SaveLayout;
    fn inflate_from_layout(&self, the_save: &SaveProject, in_file_path: &PathBuf) -> i32;

    // fn deflate_to_save(&self, save_project: &SaveProject, the_settings: &WriteSettings) -> i32;
    // fn supply_required_defaults(&self, save_project: &SaveProject) -> ();

    // protected:

    fn inflate_listing(&self, save_project: &SaveProject) -> i32;
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
    pub console: Console,
}

pub enum Console {
    Xbox360,
}

impl Xbox360Dat {
    pub fn new() -> Self {
        Xbox360Dat {
            console: Console::Xbox360,
        }
    }
}

impl ConsoleParser for Xbox360Dat {
    fn inflate_from_layout(&self, the_save: &SaveProject, in_file_path: &PathBuf) -> i32 {
        todo!()
    }

    fn inflate_listing(&self, save_project: &SaveProject) -> i32 {
        todo!()
    }
}
