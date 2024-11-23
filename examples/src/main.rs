mod player;
mod state;

use cursive::{
    event::Key,
    view::Nameable,
    views::{Dialog, TextView},
    Cursive,
};
use player::Player;
use state::{State, StoppedState};

// Application context: a music player and a state
struct PlayerApplication {
    player: Player,
    state: Box<dyn State>,
}

// https://refactoring.guru/design-patterns/state/rust/example
fn main() {
    let mut app = cursive::default();

    app.set_user_data(PlayerApplication {
        player: Player::default(),
        state: Box::new(StoppedState),
    });

    app.add_layer(
        Dialog::around(TextView::new("Press Key").with_name("歌手状态"))
            .title("网易云音乐")
            .button("播放", |s| execute(s, "播放"))
            .button("暂停", |s| execute(s, "暂停"))
            .button("上一首", |s| execute(s, "上一首"))
            .button("下一首", |s| execute(s, "下一首")),
    );
    app.add_global_callback(Key::Esc, |s| s.quit());
    app.run();
}

fn execute(s: &mut Cursive, button: &'static str) {
    let PlayerApplication {
        mut player,
        mut state,
    } = s.take_user_data().unwrap();

    let mut view = s.find_name::<TextView>("歌手状态").unwrap();

    // Here is how state mechanics work: the previous state
    // executes an action and returns a new state.
    // Each state has all 4 operations but reacts differently.
    state = match button {
        "播放" => state.play(&mut player),
        "暂停" => state.stop(&mut player),
        "上一首" => state.prev(&mut player),
        "下一首" => state.next(&mut player),
        _ => unreachable!(),
    };
    state.render(&player, &mut view);
    s.set_user_data(PlayerApplication { player, state });
}
