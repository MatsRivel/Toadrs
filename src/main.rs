use std::io::{stdin, stdout, Read, Stdin, Stdout};
use std::process::{Command, Stdio};
use std::io::Write;
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
use crossterm::event::{read,Event, KeyCode, KeyEvent, KeyEventKind};
use anyhow::{anyhow, Context};

trait CanCauseQuit{
    fn causes_quit(&self)->bool;
}
impl CanCauseQuit for Vec<u8>{
    fn causes_quit(&self)->bool { &self.len() == &1usize && &self.first() == &Some(&113u8) }
}
impl CanCauseQuit for String{
    fn causes_quit(&self)->bool { &self.len() == &1usize && &self.chars().next().expect("Already Checked") == &'q' }
}

struct KeyPressCode{
    code: KeyCode
}
impl From<KeyCode> for KeyPressCode{
    fn from(code: KeyCode) -> Self {
        Self{code}
    }
}
impl TryFrom<&Event> for KeyPressCode{
    type Error = anyhow::Error;

    fn try_from(event: &Event) -> Result<Self, Self::Error> {
        if let Event::Key(k) = event{
            if let KeyEventKind::Press = k.kind{
                return Ok(KeyPressCode::from(k.code));
            }
        };
        return Err(anyhow!("not a \"keypress\""))
    }
}


struct Editor{

}
impl Editor{
    fn new()->Self{
        Self{}
    }
    fn run(self)->anyhow::Result<()>{
        'outer: loop {
            let event = read().with_context(|| anyhow!("Failed to get event"))?;
            if let Ok(keypress) = KeyPressCode::try_from(&event){
                let c = keypress.code;
                println!("{c:?}");
                if c == KeyCode::Char('q'){
                    break 'outer;
                }
            }
        }
        anyhow::Ok(())

    }
}

fn main()->anyhow::Result<()>{
    println!("Starting...");
    let editor = Editor::new();
    editor.run().expect("We allow crashing at the upper level!");
    println!("Exiting");
    anyhow::Ok(())

}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn quit_negative(){
        let mut buffer_group = vec![];
        let strings_to_convert = vec!["quitting","hello","a",""];
        for s in strings_to_convert{
            buffer_group.push(s.as_bytes().to_vec())
        }
        for buffer in buffer_group{
            assert!(!buffer.causes_quit(),"{buffer:?} should not cause quittin!")
        }
    }
    #[test]
    fn quit_postive(){
        let buffer = "q".as_bytes().to_vec();
        assert!(buffer.causes_quit(),"{buffer:?} should not cause quittin!")

    }
}