use console::*;

use crate::time::Time;

#[derive(Clone)]
pub struct Logger {
    log_color_style: Style,
    log_time_color: Style,
    log_tags_color: Style,
    time: Time,
}

impl Logger {
    pub fn new(time: Time) -> Self {
        // Init log color style struct per log colorati
        let log_color_style = Style::new();
        // Init log time color
        let log_time_color = log_color_style.clone()
            .color256(200);
        // Init log time color
        let log_tags_color = log_color_style.clone()
            .cyan()
            .bold();

        Self {
            log_color_style,
            log_time_color,
            log_tags_color,
            time,
        }
    }

    // Metti un messaggio a schermo
    fn log(&self, log_type: StyledObject<&str>, time: String, tags: &str) {
        // Get colored time
        let ctime = self.log_time_color.apply_to(time);
        // Get colored tags
        let ctags = self.log_tags_color.apply_to(tags);
        // Print log
        println!(
            "[{}] [{}] [{}]", 
            log_type.to_string().as_str(), 
            ctime.to_string().as_str(),
            ctags.to_string().as_str()
        );
    }

    // Definisci una funzione per registrare un log di tipo "info" 
    pub fn info(&mut self, tags: &str) -> &Self {
        let green = self.log_color_style.clone().green().bright().bold();
        // Get colored log type
        let time = self.time.local_time();
        let clog_type = green.apply_to("INFO!");
        // Log
        self.log(
            clog_type, 
            time.to_ascii_uppercase(), 
            tags.to_ascii_uppercase().as_str()
        );
        self
    }

    // Definisci una funzione per registrare un log di tipo "warning" 
    pub fn warning(&mut self, tags: &str, args: &str) -> &Self {
        let yellow = self.log_color_style.clone().yellow().bright().blink().bold();
        // Get colored log type
        let time = self.time.local_time();
        let clog_type = yellow.apply_to("WARNING!");
        
        // Log
        self.log(
            clog_type, 
            time.to_ascii_uppercase(), 
            tags.to_ascii_uppercase().as_str()
        );
        println!("  {args}\n");

        self
    }

    // Definisci una funzione per registrare un log di tipo "error" 
    pub fn error(&mut self, tags: &str, err: &str) -> &Self {
        let red = self.log_color_style.clone().red().bold();
        // Get colored log type
        let time = self.time.local_time();
        let clog_type = red.apply_to("ERROR!");
        
        // Log
        self.log(
            clog_type, 
            time.to_ascii_uppercase(), 
            tags.to_ascii_uppercase().as_str()
        );
        println!("  Caused by: {err}\n");

        self
    }
}