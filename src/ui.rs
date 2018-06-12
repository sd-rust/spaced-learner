use cursive::Cursive;
use cursive::views::Dialog;
use cursive::theme::{Color, PaletteColor};

pub fn create_ui() -> Cursive {
    let mut siv = Cursive::default();

    set_theme_terminal_default(&mut siv);

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("How do you exit vim?")
        .title("Deck: Vim - Question")
        .button("Show Answer", show_answer));

    siv
}

fn show_answer(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text(":q")
        .title("Deck: Vim - Answer")
        .button("Ok", |s| s.quit()));
}


fn set_theme_terminal_default(siv: &mut Cursive) {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    siv.set_theme(theme);
}