use std::io;
use story::Story;

pub mod terminal;
pub mod block;
pub mod story;
pub mod enemy;
pub mod player;
fn main() {
    let mut story = Story::default();
    story.read();
    story.how_to_play();
    let tmp = &mut String::new();
    io::stdin().read_line(tmp).expect("Failed to read line");
    'outer: loop {
        story.intro();
        'inner: loop {
            let input = &mut String::new();
            io::stdin().read_line(input).expect("Failed to read line");
            story.choose(&input.to_lowercase());
            if story.choices.eq("end") {
                break 'inner;
            }
            story.read_block();
            if story.choices.eq("end") {
                break 'inner;
            }
        }

        let mut cont = String::new();
        story.print_lose_message();
        io::stdin().read_line(&mut cont).expect("Failed to read line");
        cont = cont.to_lowercase();
        if &cont[0..1] == "n" {
            break 'outer;
        }
    }
}