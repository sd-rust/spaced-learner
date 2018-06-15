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

extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use cursive::theme::{Color, PaletteColor, BorderStyle};
use cursive::traits::Boxable;
use cursive::align::Align;


pub fn create_ui() -> Cursive {
    let mut siv = Cursive::default();

    set_theme_terminal_default(&mut siv);

    siv.add_global_callback('q', |s| s.quit());

    let question = "How do you exit vim?";
    let answer = ":q";

    show_question(&mut siv, question, answer );

    siv
}

fn show_question(siv: &mut Cursive, question: &'static str, answer: &'static str) {
    let text_view = TextView::new(question)
                        .align(Align::center())
                        .full_screen();

    let dialog = Dialog::around(text_view)
                    .title("Vim - Question")
                    .button("Show Answer", move |s| show_answer(s, answer));

    siv.add_fullscreen_layer(dialog);
}

fn show_answer(siv: &mut Cursive, answer: &str) {
    siv.pop_layer();

    let text_view = TextView::new(answer)
        .align(Align::center())
        .full_screen();

    let dialog = Dialog::around(text_view)
        .title("Vim - Answer")
        .button("Ok", |s| s.quit());

    siv.add_fullscreen_layer(dialog);
}

fn set_theme_terminal_default(siv: &mut Cursive) {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    theme.shadow = false;
    theme.borders = BorderStyle::None;

    siv.set_theme(theme);
}