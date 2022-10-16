pub fn cursor_position(x: usize, y: usize) {
    let x = x.saturating_add(1);
    let y = y.saturating_add(1);
    print!("{}", termion::cursor::Goto(x as u16, y as u16));
}

pub fn clear_screen() {
    print!("{}", termion::clear::All)
}