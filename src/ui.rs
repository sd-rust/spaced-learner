// Copyright 2018 Sandeep Datta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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