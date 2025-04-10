use crate::game::Game;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch};
use std::{borrow::Cow, cell::RefCell, rc::Rc};

pub struct GamePrompt {
    pub game: Rc<RefCell<Game>>,
}

impl Prompt for GamePrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        let game = self.game.borrow();

        let hp = game.player.hp;
        let room = &game.current_room().name;
        Cow::Owned(format!("[HP: {}] {} > ", hp, room))
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _mode: PromptEditMode) -> Cow<str> {
        Cow::Borrowed("")
    }

    /* fn render_reversed_prompt_indicator(&self, _mode: PromptEditMode) -> Cow<str> {
        Cow::Borrowed("")
    }*/

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed("… ")
    }

    fn render_prompt_history_search_indicator(&self, _search: PromptHistorySearch) -> Cow<str> {
        Cow::Borrowed(">")
    }
}

unsafe impl Send for GamePrompt {}
