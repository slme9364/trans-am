extern crate ncurses;

mod view;
mod mode;
mod command;
mod file;
mod cursor;

use ncurses::*;
use mode::Mode;
use cursor::Cursor;
use view::View;

const LINE_NO_WINDOW: usize = 0;
const INFO_WINDOW: usize = 1;
const MAIN_WINDOW: usize = 2;
const KEY_CODE_TABLE: [&'static str; 110] = ["YES",
                                             "MIN",
                                             "Break",
                                             "Down",
                                             "Up",
                                             "Left",
                                             "Right",
                                             "Home",
                                             "BackSpace",
                                             "F0",
                                             "F1",
                                             "F2",
                                             "F3",
                                             "F4",
                                             "F5",
                                             "F6",
                                             "F7",
                                             "F8",
                                             "F9",
                                             "F10",
                                             "F11",
                                             "F12",
                                             "F13",
                                             "F14",
                                             "F15",
                                             "Dl",
                                             "Il",
                                             "Dc",
                                             "Ic",
                                             "Eic",
                                             "Clear",
                                             "EOS",
                                             "EOL",
                                             "SF",
                                             "SR",
                                             "NPage",
                                             "PPage",
                                             "STAB",
                                             "CTAB",
                                             "CATAB",
                                             "Enter",
                                             "SReset",
                                             "Reset",
                                             "Print",
                                             "LL",
                                             "A1",
                                             "A3",
                                             "B2",
                                             "C1",
                                             "C3",
                                             "BTAB",
                                             "BEG",
                                             "Cancel",
                                             "Close",
                                             "Command",
                                             "Copy",
                                             "Create",
                                             "End",
                                             "Exit",
                                             "Find",
                                             "Help",
                                             "Mark",
                                             "Message",
                                             "Move",
                                             "Next",
                                             "Open",
                                             "Options",
                                             "Previous",
                                             "Redo",
                                             "Reference",
                                             "Refresh",
                                             "Replace",
                                             "Restart",
                                             "Resume",
                                             "Save",
                                             "SBEG",
                                             "SCancel",
                                             "SCommand",
                                             "SCopy",
                                             "SCreate",
                                             "SDc",
                                             "SDl",
                                             "Select",
                                             "Send",
                                             "SEOL",
                                             "SExit",
                                             "SFind",
                                             "SHelp",
                                             "SHome",
                                             "SIc",
                                             "SLeft",
                                             "SMessage",
                                             "SMove",
                                             "SNext",
                                             "SOptions",
                                             "SPrevious",
                                             "SPrint",
                                             "SRedo",
                                             "SReplace",
                                             "SRight",
                                             "SResume",
                                             "SSave",
                                             "SSuspend",
                                             "SUndo",
                                             "Suspend",
                                             "Undo",
                                             "Mouse",
                                             "Resize",
                                             "Event",
                                             "MAX"];


fn main() {
    // Initialize
    initscr();
    let mut text = file::open_file();
    let mut view = View::new();
    view::init_view(&text, &view.windows);

    // Get key and parse and execute command
    let mut command = command::key_parse(view::get_key(view.windows[MAIN_WINDOW]));
    loop {
        view.status();
        match view.mode {
            Mode::Normal => {
                // When pushing Command is Exit Command, exit editor
                if !command::normal_exec_command(&mut command, &mut view, &text) {
                    break;
                }
            }
            Mode::Insert => {
                command::insert_exec_command(&mut command, &mut view, &mut text);
            }
        }
        wmove(view.windows[MAIN_WINDOW], view.rcursor.y, view.rcursor.x);
        command = command::key_parse(view::get_key(view.windows[MAIN_WINDOW]));
    }
    endwin();
}
