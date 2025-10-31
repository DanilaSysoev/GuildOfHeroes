use bracket_lib::prelude::{BTerm, VirtualKeyCode};

use crate::{
    core::{
        Game, MAIN_MENU_CONSOLE_INDEX,
        screens::{GlobalMapScreen, Screen},
    },
    errors::GameUiError,
    widgets::{
        geometry::Widget,
        menu::{Menu, MenuAction, MenuItem},
        rendering::MenuRenderer,
    },
};

pub struct MainMenuScreen {
    menu: Menu,
}

impl Screen for MainMenuScreen {
    fn tick(
        &mut self,
        ctx: &mut BTerm,
        game: &mut Game,
    ) -> Result<(), GameUiError> {
        ctx.set_active_console(MAIN_MENU_CONSOLE_INDEX);
        ctx.cls();

        self.create_renderer(game).render(
            &self.menu,
            ctx,
            &game.config.menu_renderer,
        );

        self.menu.select_item_if_user_requested(ctx, game);

        Ok(())
    }
}

impl MainMenuScreen {
    pub fn new() -> Result<Self, GameUiError> {
        Ok(MainMenuScreen {
            menu: Menu::new()
                .with_item(
                    VirtualKeyCode::Key1,
                    MenuItem::new("1. Start game")
                        .with_action(Box::new(ToGlobalMapAction)),
                )?
                .with_item(
                    VirtualKeyCode::Key2,
                    MenuItem::new("2. Quit game")
                        .with_action(Box::new(ExitAction)),
                )?,
        })
    }

    fn create_renderer(&mut self, game: &mut Game) -> MenuRenderer {
        MenuRenderer::new(
            ((game.height() - self.menu.height()) / 2
                - 1
                - game.config().menu_renderer.top_border) as i32,
            ((game.width() - self.menu.width()) / 2
                - 1
                - game.config().menu_renderer.left_border) as i32,
        )
    }
}

struct ToGlobalMapAction;

impl MenuAction for ToGlobalMapAction {
    fn run(
        &self,
        _: &mut bracket_lib::prelude::BTerm,
        game: &mut crate::core::Game,
    ) -> Result<(), GameUiError> {
        game.switch_screen(Box::new(GlobalMapScreen::new(game.config())?));
        Ok(())
    }
}

struct ExitAction;

impl MenuAction for ExitAction {
    fn run(
        &self,
        ctx: &mut bracket_lib::prelude::BTerm,
        _: &mut crate::core::Game,
    ) -> Result<(), GameUiError> {
        ctx.quitting = true;
        Ok(())
    }
}
