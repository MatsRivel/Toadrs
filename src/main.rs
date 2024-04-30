use std::fmt::Display;
use std::io::{self, stdin, stdout, BufRead, Read, Stdin, StdinLock, Stdout};
use std::process::{Command, Stdio};
use std::io::Write;
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use anyhow::{anyhow, Context};

struct KeyEventWrapper{
    key_event: KeyEvent
}
impl Display for KeyEventWrapper{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self.get_code(){
            KeyCode::Backspace => "<Backspace>",
            KeyCode::Enter => "<Enter>",
            KeyCode::Left => "<Left>",
            KeyCode::Right => "<Right>",
            KeyCode::Up => "<Up>",
            KeyCode::Down => "<Down>",
            KeyCode::Home => "<home>",
            KeyCode::End => "<End>",
            KeyCode::PageUp => "<PageUp>",
            KeyCode::PageDown => "<PageDown>",
            KeyCode::Tab => "<Tab>",
            KeyCode::BackTab => "<BackTab>",
            KeyCode::Delete => "<Delete>",
            KeyCode::Insert => "<insert>",
            KeyCode::F(f) => format!("<f{f}>").as_str(),
            KeyCode::Char(c) => format!("<{c}>").as_str(),
            KeyCode::Null => "<>",
            KeyCode::Esc => "<Esc>",
            KeyCode::CapsLock => "<CapsLock>",
            KeyCode::ScrollLock => "<ScrollLock>",
            KeyCode::NumLock => "<NumLock>",
            KeyCode::PrintScreen => "<PrintScreen>",
            KeyCode::Pause => "<Pause>",
            KeyCode::Menu => "<menu>",
            KeyCode::KeypadBegin => "<KeypadBegin>",
            KeyCode::Media(media) => {
                let media_str = match media{
                    event::MediaKeyCode::Play => "Play",
                    event::MediaKeyCode::Pause => "Pause",
                    event::MediaKeyCode::PlayPause => "PlayPause",
                    event::MediaKeyCode::Reverse => "Reverse",
                    event::MediaKeyCode::Stop => "Stop",
                    event::MediaKeyCode::FastForward => "FastForward",
                    event::MediaKeyCode::Rewind => "Rewind",
                    event::MediaKeyCode::TrackNext => "TrackNext",
                    event::MediaKeyCode::TrackPrevious => "TrackPrevious",
                    event::MediaKeyCode::Record => "Record",
                    event::MediaKeyCode::LowerVolume => "LowerVolume",
                    event::MediaKeyCode::RaiseVolume => "RaiseVolume",
                    event::MediaKeyCode::MuteVolume => "MuteVolume",
                };
                format!("<Media:{media_str}>").as_str()},

            KeyCode::Modifier(modifier) => {
                let modifier_str = match modifier{
                    event::ModifierKeyCode::LeftShift => "LeftShift",
                    event::ModifierKeyCode::LeftControl => "LeftCtrl",
                    event::ModifierKeyCode::LeftAlt => "LeftAlt",
                    event::ModifierKeyCode::LeftSuper => "LeftSuper",
                    event::ModifierKeyCode::LeftHyper => "LeftHyper",
                    event::ModifierKeyCode::LeftMeta => "LeftMeta",
                    event::ModifierKeyCode::RightShift => "RightShift",
                    event::ModifierKeyCode::RightControl => "RightCtrl",
                    event::ModifierKeyCode::RightAlt => "RightAlt",
                    event::ModifierKeyCode::RightSuper => "RightSuper",
                    event::ModifierKeyCode::RightHyper => "RightHyper",
                    event::ModifierKeyCode::RightMeta => "rightMeta",
                    event::ModifierKeyCode::IsoLevel3Shift => "IsoLevel3Shift",
                    event::ModifierKeyCode::IsoLevel5Shift => "IsoLevel5Shift",
                };
                format!("<Modifier{modifier_str})>").as_str()},
        };
        let kind = self.get_kind();
        let modi = self.get_modifiers();
        let stat = self.get_state();
        write!(f,"{code}")
    }
}
impl KeyEventWrapper{
    fn get_code(self)->KeyCode{
        self.key_event.code
    }
    fn get_modifiers(self)->KeyModifiers{
        self.key_event.modifiers
    }
    fn get_kind(self)->KeyEventKind{
        self.key_event.kind
    }
    fn get_state(self)->KeyEventState{
        self.key_event.state
    }
    fn is_quit(self)->bool{
        self.get_code() == KeyCode::Char('q')
    }
}
impl From<KeyEvent>for KeyEventWrapper{
    fn from(key_event: KeyEvent) -> Self {
        Self{key_event}
    }
}
impl TryFrom<Event> for KeyEventWrapper{
    type Error= anyhow::Error;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if let Event::Key(valid_key_event) = event{
                Ok(KeyEventWrapper::from(valid_key_event))
        }else{
            Err(anyhow!("Not a key event"))
        }
    }
}


// struct KeyPressCode{
//     code: KeyCode
// }
// impl From<KeyCode> for KeyPressCode{
//     fn from(code: KeyCode) -> Self {
//         Self{code}
//     }
// }
// impl From<KeyEventWrapper> for KeyPressCode{
//     fn from(kew: KeyEventWrapper) -> Self {
//         Self{code: kew.get_code()}
//     }
// }
// impl TryFrom<Event> for KeyPressCode{
//     type Error = anyhow::Error;

//     fn try_from(event: Event) -> Result<Self, Self::Error> {
//         let key_event_wrapper = KeyEventWrapper::try_from(event).with_context(||anyhow!("Could not extract KeyEvent from Event"))?;
//         Ok(KeyPressCode::from(key_event_wrapper))
//     }
// }


struct Editor{

}
impl Editor{
    fn new()->Self{
        Self{}
    }
    fn run(&self)->anyhow::Result<()>{
        'outer: loop {
            let key_event = self.read_key()?;
            if key_event.is_quit(){
                break;
            }
            
        }
        anyhow::Ok(())
    }
    fn read_key(&self)->anyhow::Result<KeyEventWrapper>{
        let event = read()?;
        KeyEventWrapper::try_from(event) 
        
        
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
}