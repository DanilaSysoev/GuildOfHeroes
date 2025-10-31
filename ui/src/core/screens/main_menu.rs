use bracket_lib::prelude::BTerm;

use crate::{
    core::{Game, MAIN_MENU_CONSOLE_INDEX, screens::Screen},
    widgets::{
        menu::{Menu, MenuAction, MenuItem},
        rendering::MenuRenderer,
    },
};

pub struct MainMenuScreen {
    menu: Menu,
}

impl Screen for MainMenuScreen {
    fn tick(&mut self, ctx: &mut BTerm, game: &mut Game) {
        ctx.set_active_console(MAIN_MENU_CONSOLE_INDEX);
        ctx.cls();

        todo!()
    }
}

impl MainMenuScreen {
    pub fn new() -> Self {
        MainMenuScreen {
            menu: Menu::new(MAIN_MENU_CONSOLE_INDEX)
                .with_item(
                    MenuItem::new("Start game")
                        .with_action(Box::new(ToGlobalMapAction)),
                )
                .with_item(
                    MenuItem::new("Quit game")
                        .with_action(Box::new(ExitAction)),
                ),
        }
    }
}

struct ToGlobalMapAction;

impl MenuAction for ToGlobalMapAction {
    fn run(
        &self,
        _: &mut bracket_lib::prelude::BTerm,
        game: &mut crate::core::Game,
    ) {
        game.switch_screen(Box::new(MainMenuScreen::new()));
    }
}

struct ExitAction;

impl MenuAction for ExitAction {
    fn run(
        &self,
        ctx: &mut bracket_lib::prelude::BTerm,
        _: &mut crate::core::Game,
    ) {
        ctx.quitting = true;
    }
}
