use ratatui::{
    layout::{Margin, Position},
    style::Color,
    widgets::Widget,
};

/// contains custom widgets

const NOTE_LOGO: &str = "                                                            
      ░████████████████████████████████████████████████░    
      ░█████████████████████████████████████████████████    
      ░██                                            ███    
      ░██                                            ███    
      ░██                                            ███    
  ░██████                                            ███    
  ░█████████░░                                       ███    
    ░░░░░░░██░    ░████████████████████████████░     ███    
      ░█████░                                        ███    
      ░██                                            ███    
      ░██                                            ███    
   ░█████          ░███████████████████████████░     ███    
  ░██░░░░          ░███████████████████████████░     ███    
   ██████████                                        ███    
      ░██████░                                       ███    
      ░██░░░                                         ███    
      ░██          ████████████████████████████      ███    
    ░ ░██           ░░░░░░░░░░░░░░░░░░░░░░░░░░       ███    
  ░██████                                            ███    
  ░█████████░                                        ███    
    ░░░░░░░██░     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░     ███    
      ░█████░      ████████████████████████████░     ███    
      ░██                                            ███    
      ░██                                            ███    
   ░█████                                            ███    
   █████░          ████████████████████████████░     ███    
  ░██████████░     ░                          ░      ███    
      ░██████░                                       ███    
      ░██  ░                                         ███    
      ░██                                            ███    
      ░██                                            ███    
      ░██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓███    
      ░████████████████████████████████████████████████░    
";

const PEN_LOGO: &str = "




         ░░░        
      ███████░      
    ░██░    ███     
    ██░      ██░    
    ███████████░    
    ███████████░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░      ██░    
    ██░░░░░░░██░    
    ███████████░    
     ░██░  ░██░     
      ░██░░██░      
       ░████░       
        ░██         
";

const NOTES_TEXT_LOGO: &str = r#"



























    ███╗   ██╗ ██████╗ ████████╗███████╗███████╗
    ████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
    ██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
    ██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
    ██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
    ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝
"#;

pub struct NotesLogo {
    note_logo: Vec<&'static str>,
    pen_logo: Vec<&'static str>,
    note_string_logo: Vec<&'static str>,
    position: u16,
}
impl NotesLogo {
    pub fn set_position(self, position: u16) -> Self {
        NotesLogo {
            position,
            note_logo: self.note_logo,
            pen_logo: self.pen_logo,
            note_string_logo: self.note_string_logo,
        }
    }
}

impl Default for NotesLogo {
    fn default() -> Self {
        let note_logo = NOTE_LOGO.lines().collect();
        let pen_logo = PEN_LOGO.lines().collect();
        let note_string_logo = NOTES_TEXT_LOGO.lines().collect();
        NotesLogo {
            note_logo,
            pen_logo,
            note_string_logo,
            position: 0,
        }
    }
}

impl Widget for NotesLogo {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let center_area = area.inner(&Margin {
            vertical: 1,
            horizontal: area.width / 2,
        });
        for (y, ((&line1, line2), line3)) in self
            .note_logo
            .iter()
            .zip(self.pen_logo)
            .zip(self.note_string_logo)
            .enumerate()
        {
            let mut right_padding = 0; // keep track of the number of position each line in first logo takes
            for (x, char) in line1.chars().enumerate() {
                let x = (self.position + center_area.left() + x as u16) % area.width;
                let y = center_area.top() + y as u16;
                buf.get_mut(x, y).set_char(char).set_fg(Color::Yellow);
                right_padding += 1;
            }
            let mut new_right_padding = 0;
            for (x, char) in line2.chars().enumerate() {
                let x =
                    (right_padding + self.position + center_area.left() + x as u16) % area.width;
                let y = center_area.top() + y as u16;
                buf.get_mut(x, y).set_char(char).set_fg(Color::Blue);
                new_right_padding += 1;
            }
            right_padding += new_right_padding;
            for (x, char) in line3.chars().enumerate() {
                let x =
                    (right_padding + self.position + center_area.left() + x as u16) % area.width;
                let y = center_area.top() + y as u16;
                buf.get_mut(x, y).set_char(char).set_fg(Color::Blue);
            }
        }
    }
}
