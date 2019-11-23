use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, SelectableList, Text, Widget};

use crate::context::Context;

use super::Commands;

type Terminal = tui::Terminal<TermionBackend<AlternateScreen<RawTerminal<std::io::Stdout>>>>;
type InterfaceResult = std::result::Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
struct Interface {
	history: Vec<String>,
	captures: Vec<String>,
	command: String,
	selected: usize,
	scroll: u16,
}

pub fn interface(context: &Context) -> InterfaceResult {
	let commands = Commands::new();
	let mut interface = Interface::default();
	interface.history.push("<initialise>".to_owned());

	let errors = std::mem::replace(context.errors.write().as_mut(), Vec::new());
	interface.captures.push(match errors.is_empty() {
		true => format!("{:#?}", context),
		false => {
			let mut capture = String::new();
			errors.into_iter().map(|diagnostic| crate::error::display(&mut capture,
				context, &diagnostic)).collect::<Result<(), _>>()?;
			capture
		}
	});

	let output = std::io::stdout().into_raw_mode()?;
	let backend = TermionBackend::new(AlternateScreen::from(output));
	let mut terminal = Terminal::new(backend)?;
	let mut input = std::io::stdin().keys();
	loop {
		render(&mut terminal, context, &interface, &commands)?;
		terminal.set_cursor(1 + interface.command.len() as u16, 1)?;

		let key = input.next();
		if let Some(key) = key.transpose()? {
			match key {
				Key::Ctrl('c') => return Ok(()),
				Key::Ctrl('u') => interface.command.clear(),
				Key::Char('\n') => {
					let command = std::mem::replace(&mut interface.command, String::new());
					let capture = commands.execute(context, &command);

					interface.history.push(command);
					interface.captures.push(capture);
					interface.selected = interface.history.len() - 1;
					interface.scroll = 0;
				}
				Key::Char(character) => interface.command.push(character),
				Key::Backspace => { interface.command.pop(); }
				Key::Down => interface.scroll += 1,
				Key::Up if interface.scroll > 0 => interface.scroll -= 1,
				Key::Ctrl('n') if interface.selected + 1 < interface.history.len() => {
					interface.selected += 1;
					interface.scroll = 0;
				}
				Key::Ctrl('p') if interface.selected > 0 => {
					interface.selected -= 1;
					interface.scroll = 0;
				}
				_ => (),
			}
		}
	}
}

fn render(terminal: &mut Terminal, context: &Context,
          interface: &Interface, commands: &Commands) -> InterfaceResult {
	Ok(terminal.draw(|mut frame| {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Ratio(2, 3),
				Constraint::Ratio(1, 3)
			].as_ref())
			.split(frame.size());
		let symbols = commands.symbols(context, &interface.command);
		List::new(symbols.into_iter().map(Text::raw))
			.block(Block::default()
				.title("Symbols")
				.borders(Borders::ALL))
			.render(&mut frame, chunks[1]);
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length(3),
				Constraint::Ratio(1, 5),
				Constraint::Min(0)
			].as_ref())
			.split(chunks[0]);
		let command = Text::raw(&interface.command);
		Paragraph::new(std::iter::once(&command))
			.block(Block::default()
				.title("Command")
				.borders(Borders::ALL))
			.render(&mut frame, chunks[0]);
		SelectableList::default()
			.block(Block::default()
				.title("History")
				.borders(Borders::ALL))
			.items(&interface.history)
			.select(Some(interface.selected))
			.highlight_style(Style::default()
				.bg(Color::White)
				.fg(Color::Black))
			.render(&mut frame, chunks[1]);
		let capture = Text::raw(&interface.captures[interface.selected]);
		Paragraph::new(std::iter::once(&capture))
			.block(Block::default()
				.title("Capture")
				.borders(Borders::ALL))
			.scroll(interface.scroll)
			.render(&mut frame, chunks[2]);
	})?)
}
