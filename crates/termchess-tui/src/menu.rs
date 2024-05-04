use {
    super::{ascii, Screen, Terminal},
    crate::TResult,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEventKind},
    items::Item,
    ratatui::{
        layout::Position,
        prelude::*,
        symbols::border,
        widgets::{
            block::{Position as TermPosition, Title},
            Block, Borders, Paragraph,
        },
    },
    std::{cell::RefCell, rc::Rc},
    strum::{EnumCount, IntoEnumIterator},
};

pub mod items;

#[derive(Debug)]
pub struct Menu {
    current: usize,
    exit: bool,
    // this is a gross solution, but for now it'll stay
    items_pos: Position,
    mouse_pos: Position,
    terminal: Rc<RefCell<Terminal>>,
}

impl Menu {
    pub fn new(terminal: Rc<RefCell<Terminal>>) -> Self {
        Self {
            terminal,
            current: 0,
            exit: false,
            items_pos: Position::default(),
            mouse_pos: Position::default(),
        }
    }

    fn mouse_over_item(&self) -> Option<Item> {
        match self.mouse_pos.y as i32 - self.items_pos.y as i32 - 2 {
            // 2 = space before items
            y if (0..Item::COUNT).contains(&(y as usize)) => {
                let item = Item::from_repr(y as usize).expect("see line above");

                let len = item.to_string().len() as i32;
                let dis = (60 - len) / 2;
                if (dis..(dis + len)).contains(&(self.mouse_pos.x as i32 - self.items_pos.x as i32))
                {
                    Some(item)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn items_widget(&self) -> Paragraph {
        let title = Title::from(" termchess ".bold());
        let instructions = Title::from(Line::from(vec![
            " Up ".into(),
            "[<Up> | j] ".blue().bold(),
            "Down ".into(),
            "[<Down> | k] ".blue().bold(),
            "Select ".into(),
            "<Enter> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(TermPosition::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let items_unstyled: Vec<String> = Item::iter().map(|item| item.to_string()).collect();
        let mut items_styled = Vec::new();
        items_styled.push(Line::from("\n"));
        for (i, item) in items_unstyled.iter().enumerate() {
            items_styled.push(Line::from({
                let item = if i == self.current as usize {
                    format!("<{}>", item).magenta() //.bold()
                } else {
                    item.clone().white() // can't move out of Vec
                };
                if Some(Item::from_repr(i).expect("see items_unstyled")) == self.mouse_over_item() {
                    item.bold()
                } else {
                    item
                }
            }));
            // adds some spacing between items
            // items_styled.push("\n".into());
        }

        Paragraph::new(Text::from(items_styled))
            .centered()
            .block(block)
    }
}

impl Screen for Menu {
    fn render_frame(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(27),
                Constraint::Fill(2),
                Constraint::Length((Item::COUNT + 4) as u16),
                Constraint::Fill(1),
            ])
            .split(frame.size());

        let canvas_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(80),
                Constraint::Fill(1),
            ])
            .split(layout[1]);

        let items_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(60),
                Constraint::Fill(1),
            ])
            .split(layout[3]);

        self.items_pos = items_layout[1].as_position();

        frame.render_widget(art_widget(), canvas_layout[1]);
        frame.render_widget(self.items_widget(), items_layout[1]);
    }

    fn handle_key(&mut self, key: KeyEvent) -> TResult<()> {
        if key.modifiers == KeyModifiers::NONE {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.current > 0 {
                        self.current -= 1
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.current < (Item::COUNT - 1) {
                        self.current += 1
                    }
                }
                KeyCode::Enter => {
                    Item::from_repr(self.current)
                        .expect("should be within bounds")
                        .handle(self);
                }
                _ => (),
            }
        }
        if key.modifiers == KeyModifiers::CONTROL {
            match key.code {
                KeyCode::Char('c') => self.exit = true,
                _ => (),
            }
        }

        Ok(())
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) -> TResult<()> {
        self.mouse_pos = Position::new(mouse.column, mouse.row);
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if let Some(item) = self.mouse_over_item() {
                item.handle(self);
            }
        }

        Ok(())
    }

    fn handle_misc(&mut self, event: Event) -> TResult<()> {
        // nothing to do here (for now)

        Ok(())
    }

    fn terminal(&self) -> &Rc<RefCell<Terminal>> {
        &self.terminal
    }

    fn exit(&self) -> bool {
        self.exit
    }
}

// impl Default for Menu {
//     fn default() -> Self {
//         Self {
//             current: 0,
//             exit: false,
//             items_pos: Position::default(),
//             mouse_pos: Position::default(),
//         }
//     }
// }

fn art_widget() -> Paragraph<'static> {
    let text = Text::from(ascii::MENU_ART);

    Paragraph::new(text).centered()
}
