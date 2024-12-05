use std::{
    io::{stderr, stdout, Result, Write},
    time::Instant,
};

/// Handles the Printing logic for the Spinner
#[derive(Default, Clone)]
pub enum Stream {
    #[default]
    Stderr,
    Stdout,
    #[cfg(feature = "ratatui")]
    Ratatui(crate::SpinnerWidget),
}

impl Stream {
    /// Writes the current message and optionally prints the durations
    pub fn write(
        &mut self,
        frame: &str,
        message: &str,
        start_time: Option<Instant>,
        stop_time: Option<Instant>,
    ) -> Result<()> {
        match start_time {
            None => self.print(frame, message)?,
            Some(start_time) => self.print_with_duration(frame, message, start_time, stop_time)?,
        }
        Ok(())
    }

    /// Writes the message without duration
    fn print(&mut self, frame: &str, message: &str) -> Result<()> {
        let mut writer = self.writer();
        self.start_of_line()?;
        write!(writer, "{frame} {message}")?;
        writer.flush()
    }

    /// Writes the message with the duration
    fn print_with_duration(
        &mut self,
        frame: &str,
        message: &str,
        start_time: Instant,
        stop_time: Option<Instant>,
    ) -> Result<()> {
        let mut writer = self.writer();
        let now = stop_time.unwrap_or_else(Instant::now);
        let duration = now.duration_since(start_time).as_secs_f64();
        write!(writer, "\r{frame}{duration:>10.3} s\t{message}")?;
        writer.flush()
    }

    /// Handles the stopping logic given an optional message and symbol
    pub fn stop(&self, message: Option<&str>, symbol: Option<&str>) -> Result<()> {
        let mut writer = self.writer();
        match (message, symbol) {
            (Some(message), Some(symbol)) => {
                self.erase_line()?;
                writeln!(writer, "{symbol} {message}")
            }
            (Some(message), None) => {
                self.erase_line()?;
                writeln!(writer, "{message}")
            }
            _ => writeln!(writer),
        }?;
        writer.flush()
    }

    fn start_of_line(&self) -> Result<()> {
        match self {
            #[cfg(feature = "ratatui")]
            Self::Ratatui(widget) => {
                widget.clear();
                Ok(())
            }
            _ => {
                let mut writer = self.writer();
                write!(writer, "\r")?;
                writer.flush()
            }
        }
    }

    fn erase_line(&self) -> Result<()> {
        match self {
            #[cfg(feature = "ratatui")]
            Self::Ratatui(widget) => {
                widget.clear();
                Ok(())
            }
            _ => {
                let mut writer = self.writer();
                write!(writer, "\x1b[2K\r")?;
                writer.flush()
            }
        }
    }

    /// Matches on self and returns the internal writer
    fn writer(&self) -> Box<dyn Write> {
        match self {
            Self::Stderr => Box::new(stderr()),
            Self::Stdout => Box::new(stdout()),
            #[cfg(feature = "ratatui")]
            Self::Ratatui(widget) => Box::new(widget.clone()),
        }
    }
}
