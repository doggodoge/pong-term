use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::{io::stdout, time::Duration};

const WIDTH: u16 = 80;
const HEIGHT: u16 = 20;
const PADDLE_HEIGHT: u16 = 6;

struct Player {
    y: u16,
}

impl Player {
    fn new(y: u16) -> Player {
        Player { y }
    }
}

struct Ball {
    x: u16,
    y: u16,
    dx: i16,
    dy: i16,
}

impl Ball {
    fn new(x: u16, y: u16) -> Ball {
        Ball {
            x,
            y,
            dx: -1,
            dy: 1,
        }
    }
}

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();

    let mut left_player = Player::new(HEIGHT / 2 - PADDLE_HEIGHT / 2);
    let mut right_player = Player::new(HEIGHT / 2 - PADDLE_HEIGHT / 2);
    let mut ball = Ball::new(WIDTH / 2, HEIGHT / 2);

    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => left_player.y = left_player.y.saturating_sub(1),
                    KeyCode::Char('s') => {
                        left_player.y = (left_player.y + 1).min(HEIGHT - PADDLE_HEIGHT)
                    }
                    KeyCode::Char('i') => right_player.y = right_player.y.saturating_sub(1),
                    KeyCode::Char('k') => {
                        right_player.y = (right_player.y + 1).min(HEIGHT - PADDLE_HEIGHT)
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        ball.x = (ball.x as i16 + ball.dx) as u16;
        ball.y = (ball.y as i16 + ball.dy) as u16;

        if ball.y == 0 || ball.y == HEIGHT - 1 {
            ball.dy = -ball.dy;
        }

        if (ball.x == 1 && (left_player.y..left_player.y + PADDLE_HEIGHT).contains(&ball.y))
            || (ball.x == WIDTH - 2
                && (right_player.y..right_player.y + PADDLE_HEIGHT).contains(&ball.y))
        {
            ball.dx = -ball.dx;
        }

        if ball.x == 0 || ball.x == WIDTH - 1 {
            ball = Ball::new(WIDTH / 2, HEIGHT / 2);
        }

        stdout.execute(Clear(ClearType::All)).unwrap();

        for i in 0..HEIGHT {
            stdout
                .execute(MoveTo(0, i))
                .unwrap()
                .execute(SetForegroundColor(Color::White))
                .unwrap()
                .execute(Print("|"))
                .unwrap()
                .execute(MoveTo(WIDTH - 1, i))
                .unwrap()
                .execute(Print("|"))
                .unwrap()
                .execute(ResetColor)
                .unwrap();
        }

        for i in 0..PADDLE_HEIGHT {
            stdout
                .execute(MoveTo(1, left_player.y + i))
                .unwrap()
                .execute(SetBackgroundColor(Color::Blue))
                .unwrap()
                .execute(Print(" "))
                .unwrap()
                .execute(ResetColor)
                .unwrap();

            stdout
                .execute(MoveTo(WIDTH - 2, right_player.y + i))
                .unwrap()
                .execute(SetBackgroundColor(Color::Blue))
                .unwrap()
                .execute(Print(" "))
                .unwrap()
                .execute(ResetColor)
                .unwrap();
        }

        stdout
            .execute(MoveTo(ball.x, ball.y))
            .unwrap()
            .execute(SetForegroundColor(Color::Red))
            .unwrap()
            .execute(Print("O"))
            .unwrap()
            .execute(ResetColor)
            .unwrap();
    }

    disable_raw_mode().unwrap();
}
