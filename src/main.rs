use std::fmt::Error;
use std::io::{stdin, Read, Stdin};
use std::process::{Command, Stdio};
use std::io::Write;

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
fn main()->anyhow::Result<()>{
    
    loop {
        let mut buffer: String = String::new();
        stdin().read_to_string(&mut buffer).with_context(|| anyhow!("Failed to read input. Terminating."))?;
        if buffer.causes_quit(){
            break;
        }
        println!("{buffer:?}");
    }
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