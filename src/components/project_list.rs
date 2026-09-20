use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Offset, Rect},
    style::Style,
    widgets::{Block, Paragraph},
};

use crate::{
    components::Component,
    state::app_state::AppState,
    theme::{create_base_highlighted_block, get_highlight_color},
};

pub struct ProjectList {}

impl ProjectList {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for ProjectList {
    fn draw(&mut self, frame: &mut Frame, area: Rect, state: &mut AppState) {
        let block = create_base_highlighted_block()
            .title(" Projects ")
            .title_alignment(Alignment::Left);
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let rows = state.projects.len().min(inner_area.height as usize);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); rows])
            .split(inner_area);

        for (i, project) in state.projects.iter().take(rows).enumerate() {
            let style = if i == state.project_focus {
                get_highlight_color()
            } else {
                Style::default()
            };

            frame.render_widget(Block::new().style(style), layout[i]);
            frame.render_widget(
                Paragraph::new(project.name.as_str()),
                layout[i].offset(Offset { x: 1, y: 0 }),
            );
        }
    }

    fn get_children_layout(&self) -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1)])
    }
}
