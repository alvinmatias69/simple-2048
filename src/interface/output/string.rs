use super::interface::OutputInterface;

pub struct OutputString {}

impl OutputInterface for OutputString {
    fn show(&self, list: &Vec<Vec<u32>>) {
        for row in list.iter() {
            for item in row.iter() {
                print!("[{}] ", item);
            }
            println!("");
        }
    }
}
