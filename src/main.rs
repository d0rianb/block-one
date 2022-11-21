mod context;
mod animation;
mod block;
mod render_helper;

#[macro_use]
extern crate derivative;
extern crate core;

use std::thread;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::time::{Duration, Instant};

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::{KeyScancode, ModifiersState, MouseButton, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper, WindowPosition, WindowSize, WindowStartupInfo};
use speedy2d::{Graphics2D, Window};

use crate::context::Context;

const FPS: u64 = 60;
const FRAME_DURATION: u64 = 1000 / FPS; // ms

#[derive(PartialEq, Debug, Clone)]
pub enum AppEvent {
    Update,
    Redraw,
}

struct AppWindowHandler {
    context: Context,
    tick_timestamp: Instant,
    mouse_button_pressed: (bool, bool), // (Left, Right)
}

impl WindowHandler<AppEvent> for AppWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper<AppEvent>, _info: WindowStartupInfo) {
        let event_sender = helper.create_user_event_sender();
        helper.request_redraw();
        thread::spawn(move || {
            loop {
                event_sender.send_event(AppEvent::Update).unwrap();
                thread::sleep(Duration::from_millis(FRAME_DURATION));
            }
        });
    }

    #[warn(unreachable_patterns)]
    fn on_user_event(&mut self, helper: &mut WindowHelper<AppEvent>, user_event: AppEvent) {
        match user_event {
            AppEvent::Redraw => helper.request_redraw(),
            AppEvent::Update => {
                self.context.update(self.tick_timestamp.elapsed().as_millis() as f32);
                self.tick_timestamp = Instant::now();
            },
        }
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<AppEvent>, size_pixels: Vector2<u32>) {}

    fn on_draw(&mut self, _helper: &mut WindowHelper<AppEvent>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        self.context.render(graphics);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<AppEvent>, position: Vector2<f32>) {
        self.context.mouse_position = position;
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<AppEvent>, button: MouseButton) {
        self.context.on_keypress(button);
        helper.request_redraw();
    }

    fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper<AppEvent>, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse_button_pressed.0 = false,
            MouseButton::Right => self.mouse_button_pressed.1 = false,
            _ => ()
        }
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<AppEvent>, virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        // if let Some(keycode) = virtual_key_code {
        //     match self.focus {
        //         FocusElement::Menu(id) => self.editor.get_menu(id).handle_key(keycode, modifiers),
        //         FocusElement::Editor => self.editor.handle_key(keycode),
        //         FocusElement::MenuInput(id) => self.editor.get_menu(id).send_key_to_input(keycode, modifiers),
        //     }
        // }
        helper.request_redraw();
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<AppEvent>, unicode_codepoint: char) {
        if unicode_codepoint >= ' '  && unicode_codepoint <= '~' || unicode_codepoint >= '¡' {
            self.context.on_keydown(unicode_codepoint.to_string());
            // match self.focus {
            //     FocusElement::Editor => {
            //         self.editor.add_char(unicode_codepoint.to_string());
            //         self.editor.update_text_layout();
            //     }
            //     FocusElement::MenuInput(id) => {
            //         let input = self.editor.get_menu(id).get_focused_item().input.as_mut().unwrap();
            //         input.add_char(unicode_codepoint.to_string());
            //         input.update_text_layout();
            //     }
            //     FocusElement::Menu(id) => {
            //         // Cancel chip should disapear on keydown but the char should be added anyway
            //         // Ugly
            //         let menu = self.editor.get_menu(id);
            //         if menu.items[0].action == MenuAction::CancelChip {
            //             self.editor.add_char(unicode_codepoint.to_string());
            //             self.editor.update_text_layout();
            //         }
            //     }
            // }
            helper.request_redraw();
        }
    }

    fn on_keyboard_modifiers_changed(&mut self, _helper: &mut WindowHelper<AppEvent>, state: ModifiersState) { }
}

fn set_app_title(helper: &mut WindowHelper<AppEvent>, path: &str) {
    helper.set_title("Block One")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // For transparenting the titlebar : set
    //      ns_window.setTitlebarAppearsTransparent_(YES);
    //      masks |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
    let window = Window::new_with_user_events(
        "Block One",
        WindowCreationOptions::new_windowed(
            WindowSize::ScaledPixels((600., 400.).into()),
            Some(WindowPosition::Center)
        )
    ).unwrap();
    let mut context = Context::new();
    if args.len() > 1 { }

    let window_handler = AppWindowHandler {
        context,
        tick_timestamp: Instant::now(),
        mouse_button_pressed: (false, false),
    };

    window.run_loop(window_handler);
}
