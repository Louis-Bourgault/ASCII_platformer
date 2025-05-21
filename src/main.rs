use crossterm::{
    cursor, event,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::time::Instant;
use std::{
    io::{stdout, Write},
    time::Duration,
};
struct Obstacle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

struct Player {
    x: i32,
    y: i32,
}

struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?; // stops input from being printed

    let mut player: Player = Player { x: 0, y: 3 };

    let obstacles: Vec<Obstacle> = vec![
        Obstacle {
            x: 0,
            y: 5,
            width: 10,
            height: 1,
        },
        Obstacle {
            x: 13,
            y: 5,
            width: 10,
            height: 1,
        },
        Obstacle {
            x: 26,
            y: 3,
            width: 10,
            height: 1,
        },
        Obstacle {
            x: 39,
            y: 1,
            width: 10,
            height: 1,
        },
        Obstacle {
            x: 52,
            y: -1,
            width: 10,
            height: 1,
        },
    ];

    let now = std::time::Instant::now();
    let mut last_update: Instant = now;
    let mut last_fall: Instant = now;

    loop {
        let now = Instant::now();
        let elapsed = now - last_update;
        let mut view = Viewport {
            x: 0,
            y: 0,
            width: 100,
            height: 30,
        };
        last_update = now;

        let mut input_direction_x = 0;
        let mut jump_pressed = false;
        if event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Char('a') => {
                        input_direction_x = -1;
                    }
                    event::KeyCode::Char('d') => {
                        if input_direction_x == -1 {
                            input_direction_x = 0;
                        } else {
                            input_direction_x = 1
                        }; // Move right
                    }
                    event::KeyCode::Char('w') => {
                        jump_pressed = true;
                    }
                    event::KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        handle_player_movement(
            &mut player,
            input_direction_x,
            jump_pressed,
            &obstacles,
            &mut last_fall,
        );
        view = calculate_viewport(&player, &view);
        render_screen(&player, &obstacles, &view)?;
        println!("{}", elapsed.as_secs_f32());
        let fps = 60.0 / elapsed.as_secs_f32();
        println!("Running at {} fps", fps);
        println!("{}", check_if_solid_ground(&player, &obstacles))
    }

    // Clean up the terminal
    terminal::disable_raw_mode()?;
    stdout().execute(cursor::Show)?;

    Ok(())
}

fn handle_player_movement(
    player: &mut Player,
    input_direction_x: i32,
    jump_pressed: bool,
    obstacles: &Vec<Obstacle>,
    last_fall: &mut Instant,
) {
    player.x += input_direction_x;
    if !check_if_solid_ground(player, obstacles) {
        if Instant::now() - *last_fall > Duration::from_millis(300) {
            player.y += 1;
            *last_fall = Instant::now();
        }
    } else {
        if jump_pressed {
            player.y -= 4;
        }
    }
}

fn check_if_solid_ground(player: &Player, obstacles: &Vec<Obstacle>) -> bool {
    for obstacle in obstacles {
        if player.y == obstacle.y - 1
            && player.x >= obstacle.x
            && player.x < obstacle.x + obstacle.width
        {
            return true;
        }
    }
    false
}

fn calculate_viewport(player: &Player, viewport: &Viewport) -> Viewport {
    let mut new_viewport = Viewport {
        x: viewport.x,
        y: viewport.y,
        width: viewport.width,
        height: viewport.height,
    };

    if player.x - viewport.x < 5 {
        new_viewport.x = player.x - 5;
    }
    if player.x - viewport.x > 45 {
        new_viewport.x = player.x - 45;
    }
    if player.y - viewport.y < 5 {
        new_viewport.y = player.y - 5;
    }
    if player.y - viewport.y > 5 {
        new_viewport.y = player.y - 5;
    }

    new_viewport
}

fn render_screen(
    player: &Player,
    obstacles: &Vec<Obstacle>,
    viewport: &Viewport,
) -> crossterm::Result<()> {
    let mut stdout = stdout();

    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    for i in 0..viewport.height {
        let real_y = i + viewport.y;
        for j in 0..viewport.width {
            let real_x = j + viewport.x;

            let mut obstacle = false;
            for obs in obstacles {
                if real_y == obs.y && real_x >= obs.x && real_x < obs.x + obs.width {
                    obstacle = true;
                }
                if real_y == obs.y + obs.height && real_x >= obs.x && real_x < obs.x + obs.width {
                    obstacle = true;
                }
            }
            if real_y == player.y && real_x == player.x {
                print!("P");
            } else if obstacle {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("{}", player.x);
    println!("{}", player.y);

    stdout.flush()?; // Ensure everything is printed to the terminal
    Ok(())
}
