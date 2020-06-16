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
use cursive::CursiveExt;
use cursive::views::{Dialog, TextView};
use cursive::theme::{Color, PaletteColor, BorderStyle};
use cursive::traits::Boxable;
use cursive::align::Align;
use crate::model;

pub fn create_ui() -> Cursive {
    let mut siv = Cursive::default();

    set_theme_terminal_default(&mut siv);

    siv.add_global_callback('q', |s| s.quit());

    siv
}

pub fn start_quiz(siv: &mut Cursive) {
    show_question(siv, 0);
}

pub fn show_question(siv: &mut Cursive, question_num: usize) {

    let quiz: &mut model::Quiz = siv.user_data().unwrap();

    if question_num >= quiz.bank.len() {
        siv.quit();
        return;
    }

    let qna = &quiz.bank[question_num];

    let tv_question = TextView::new(&qna.question)
                        .align(Align::center())
                        .full_screen();

    let dlg_question = Dialog::around(tv_question)
        .title(&quiz.title)
        .button("Show Answer", move |s| show_answer(s, question_num));

    siv.add_fullscreen_layer(dlg_question);
}

pub fn show_answer(siv: &mut Cursive, question_num: usize) {
    siv.pop_layer();

    let quiz: &mut model::Quiz = siv.user_data().unwrap();

    let qna = &quiz.bank[question_num];

    let tv_answer = TextView::new(&qna.answer)
        .align(Align::center())
        .full_screen();

    let dlg_answer = Dialog::around(tv_answer)
        .title(&quiz.title)
        .button("Ok", move |s| show_question(s, question_num + 1));

    siv.add_fullscreen_layer(dlg_answer);
}

fn set_theme_terminal_default(siv: &mut Cursive) {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    theme.shadow = false;
    theme.borders = BorderStyle::None;

    siv.set_theme(theme);
}