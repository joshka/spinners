use std::{
    io::{Result, Write},
    sync::{Arc, Mutex},
};

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::Stream;

/// A ratatui widget that can be used to display a spinner
///
/// # Examples
///
/// ```no_run
/// use ratatui::widgets::SpinnerWidget;
///
/// let spinner_widget = SpinnerWidget::default();
/// let spinner = Spinner::with_stream(
///     Spinners::Dots9,
///     "Waiting ".into(),
///     spinner_widget.as_steam(),
/// );
/// # let area = Rect::default();
/// # let buf = &mut Buffer::empty(area);
/// spinner_widget.render(area, buf);
/// ```
#[derive(Clone, Default)]
pub struct SpinnerWidget {
    value: Arc<Mutex<Vec<u8>>>,
}

impl SpinnerWidget {
    pub(crate) fn clear(&self) {
        self.value.lock().unwrap().clear();
    }

    pub fn as_steam(&self) -> Stream {
        Stream::Ratatui(self.clone())
    }
}

impl Write for SpinnerWidget {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut vec = self.value.lock().unwrap();
        vec.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Widget for &SpinnerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let value = self.value.lock().unwrap();
        let value = std::str::from_utf8(&value).unwrap_or_default();
        value.render(area, buf);
    }
}
