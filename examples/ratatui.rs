use std::{
    thread,
    time::{Duration, Instant},
};

use ratatui::DefaultTerminal;
use spinners::{Spinner, SpinnerWidget, Spinners};

fn main() {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result.expect("error running ratatui");
}

fn run(mut terminal: DefaultTerminal) -> std::io::Result<()> {
    let spinner_widget = SpinnerWidget::default();
    let mut spinner = Spinner::with_stream(
        Spinners::Dots9,
        "Waiting for 3 seconds".into(),
        spinner_widget.as_steam(),
    );

    const FPS: f64 = 60.0;
    let frame_duration = Duration::from_secs_f64(1.0 / FPS);

    // Normally a Rataui app would have a loop that interacts with user events, but for this example
    // we just want to show the spinner for 3 seconds, then stop it, making sure to render the
    // spinner in the terminal each frame
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(3) {
        terminal.draw(|frame| frame.render_widget(&spinner_widget, frame.area()))?;
        thread::sleep(frame_duration);
    }

    spinner.stop_with_message("Finishing waiting for 3 seconds\n".into());

    // show the finished spinner for 3 more seconds
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(3) {
        terminal.draw(|frame| frame.render_widget(&spinner_widget, frame.area()))?;
        thread::sleep(frame_duration);
    }
    Ok(())
}
