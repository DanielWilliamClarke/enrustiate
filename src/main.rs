use tone_gen::{Sheet, SheetPlayer, SheetPlayerOption};

fn main() {
    let sheet = Sheet::from((10, 4));
    let mut player = SheetPlayer::from(sheet);

    loop {
        player.play();

        match player.read_input() {
            Ok(option) => match option {
                SheetPlayerOption::Play => (),
                SheetPlayerOption::Exit => {
                    println!("See you next time!");
                    break;
                }
            }
            Err(err) =>   println!("{}", err)
        }
    }
}
