use std::{
    io::{self, Stdout},
    time::Duration,
    error::Error
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    Terminal,
    prelude::*,
    widgets::{
        Block,
        Borders,
        BorderType,
        Paragraph,
        block::{Title, Position}, Wrap
    }};
use reqwest::Client;
use tokio::task::JoinSet;

use crate::address::FinalWallet;
use crate::generator::generate_batches;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(terminal.show_cursor()?)
}

pub async fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, client: Client) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        let mut tasks = JoinSet::new();
        for _ in 0..5 {
            tasks.spawn(generate_batches(client.clone()));
        }

        while let Some(res) = tasks.join_next().await {
            for result in res.unwrap().unwrap() {
                terminal.draw(|f| ui(f, result))?;
            }
        }

        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    })
}

fn ui(frame: &mut Frame<CrosstermBackend<Stdout>>, data: FinalWallet) {
    let [upmost,top, mid, bottom] = *Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3),
                Constraint::Min(4),
                Constraint::Min(13),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .split(frame.size())
    else {
        return;
    };

    let balance_vertical = Layout::default()
        .direction(Direction::Horizontal)
        .horizontal_margin(3)
        .vertical_margin(2)
        .constraints(vec![Constraint::Percentage(20); 5])
        .split(mid);

    let balance_horizontal: Vec<_> = balance_vertical
        .iter()
        .flat_map(|area| {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Max(3); 3])
                .split(*area)
                .to_vec()
        })
        .collect();

    // TODO: Optimize to avoid using clone.
    render_text(data.address.clone(), Color::Green, Color::Cyan, "Address", frame, upmost);
    render_text(data.mnemonic.clone(), Color::Green, Color::Cyan,"Mnemonic", frame, top);
    render_text(data.balances.bsc.clone(), Color::White, Color::Blue, "BSC", frame, balance_horizontal[0]);
    render_text(data.balances.ethereum.clone(), Color::White, Color::Blue, "Ethereum", frame, balance_horizontal[1]);
    render_text(data.balances.polygon.clone(), Color::White, Color::Blue, "Polygon", frame, balance_horizontal[2]);
    render_text(data.balances.polygon_zkevm.clone(), Color::White, Color::Blue, "Polygon zkEVM", frame, balance_horizontal[3]);
    render_text(data.balances.arbitrum.clone(), Color::White, Color::Blue, "Arbitrum", frame, balance_horizontal[4]);
    render_text(data.balances.fantom.clone(), Color::White, Color::Blue, "Fantom", frame, balance_horizontal[5]);
    render_text(data.balances.optimism.clone(), Color::White, Color::Blue, "Optimism", frame, balance_horizontal[6]);
    render_text(data.balances.cronos.clone(), Color::White, Color::Blue, "Cronos", frame, balance_horizontal[7]);
    render_text(data.balances.bittorrent.clone(), Color::White, Color::Blue, "Bittorrent", frame, balance_horizontal[8]);
    render_text(data.balances.moonbeam.clone(), Color::White, Color::Blue, "Moonbeam", frame, balance_horizontal[9]);
    render_text(data.balances.moonriver.clone(), Color::White, Color::Blue, "Moonriver", frame, balance_horizontal[10]);
    render_text(data.balances.avalanche.clone(), Color::White, Color::Blue, "Avalanche", frame, balance_horizontal[11]);
    render_text(data.balances.celo.clone(), Color::White, Color::Blue, "Celo", frame, balance_horizontal[12]);
    render_text(data.balances.gnosis.clone(), Color::White, Color::Blue, "Gnosis", frame, balance_horizontal[13]);

    // frame.render_widget(
    //     Paragraph::new("Constraint::Length(4)").block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .border_type(BorderType::Rounded)
    //             .title(
    //                 Title::from("Ethereum Address")
    //                     .position(Position::Top)
    //                     .alignment(Alignment::Left)
    //             )
    //     ),
    //     upmost,
    // );

    // frame.render_widget(
    //     Paragraph::new("Constraint::Length(4)").block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .border_type(BorderType::Rounded)
    //             .title(
    //                 Title::from("Ethereum Address")
    //                     .position(Position::Top)
    //                     .alignment(Alignment::Left)
    //             )
    //     ),
    //     top,
    // );

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Balance"),
        mid,
    );

    // frame.render_widget(
    //     Paragraph::new("Constraint::Ratio(2, 5)\nhorizontal_margin(5)\nvertical_margin(2)")
    //         .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)),
    //     left,
    // );
    // frame.render_widget(
    //     Paragraph::new("Constraint::Ratio(3, 5)").block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)),
    //     right,
    // );

    frame.render_widget(
        Paragraph::new(
            Span::styled("No wallets found yet", Style::default().fg(Color::DarkGray)))
            .fg(Color::Yellow)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Wallets Found")),
        bottom,
    );
}

pub fn render_text(paragraph: String, color: Color, frame_color: Color, title: &str, frame: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
    frame.render_widget(
        Paragraph::new(Span::styled(paragraph, Style::default().fg(color)))
            .fg(frame_color)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(
                        Title::from(title)
                            .position(Position::Top)
                            .alignment(Alignment::Left)
                )
            )
            .wrap(Wrap { trim: true }),
        area,
    );
}
