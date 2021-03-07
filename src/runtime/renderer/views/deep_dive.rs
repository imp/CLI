use tui::Terminal;
use tui::backend::CrosstermBackend;
use std::io::Stdout;
use crate::runtime::data::launches::structures::{Launch, Rocket};
use tui::layout::{Layout, Direction, Constraint, Alignment, Rect};
use tui::widgets::{Clear, Block, Borders, Paragraph};
use tui::text::Text;

pub mod dict;

pub fn run(mut out: Terminal<CrosstermBackend<Stdout>>, launch_present: bool, i: &Option<Launch>) {
    if launch_present {

        let ln = i.clone().unwrap();
        let rocket = ln.rocket.unwrap_or(Rocket {
            id: None,
            configuration: None
        });
        let vehicle_id = dict::match_rocket(rocket.id.unwrap_or(0));

        if vehicle_id.is_some() {
            out.draw(|mut f| {
                let size = f.size();

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(size);

                let paragraph = Paragraph::new(Text::raw(""))
                    .block(Block::default().title(" Detailed Overview ").borders(Borders::ALL))
                    .alignment(Alignment::Left);
                f.render_widget(paragraph, chunks[0]);

                let paragraph = Paragraph::new(Text::raw(""))
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Left);
                f.render_widget(paragraph, chunks[1]);
            });
        } else {
            out.draw(|mut f| {
                let size = f.size();

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(size);

                let paragraph = Paragraph::new(Text::raw(""))
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Left);
                f.render_widget(paragraph, chunks[0]);

                let paragraph = Paragraph::new(Text::raw(""))
                    .block(Block::default().title("Detailed Overview").borders(Borders::ALL))
                    .alignment(Alignment::Left);
                f.render_widget(paragraph, chunks[1]);

                let error = Paragraph::new(Text::raw("There is currently no detailed overview available for this rocket.\nIt may be coming in a future patch."))
                    .block(Block::default().title("Popup").borders(Borders::ALL))
                    .alignment(Alignment::Left);

                let area = centered_rect(60, 20, size);

                f.render_widget(Clear, area); //this clears out the background
                f.render_widget(error, area);
            });
        }


    } else {
        out.draw(|mut f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            let paragraph = Paragraph::new(Text::raw(""))
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[0]);

            let paragraph = Paragraph::new(Text::raw(""))
                .block(Block::default().title("Detailed Overview").borders(Borders::ALL))
                .alignment(Alignment::Left);
            f.render_widget(paragraph, chunks[1]);

            let error = Paragraph::new(Text::raw("There is currently no launch available.\nCheck this screen again once one is available."))
                .block(Block::default().title("Popup").borders(Borders::ALL))
                .alignment(Alignment::Left);

            let area = centered_rect(60, 20, size);

            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(error, area);
        });
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
                .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
                .as_ref(),
        )
        .split(popup_layout[1])[1]
}