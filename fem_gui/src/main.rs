use fem_comp::{self, vals::Obj};
use raylib::{
    ffi::{Rectangle, Vector2},
    prelude::*,
};

use nfd2::Response;

use std::{ffi::CString, path::Path};

fn static_cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}
// TODO create a basic gui that inputs data , outputs raw information about the object and quits
fn main() {
    let layout_recs = [
        Rectangle {
            x: 24.0,
            y: 120.0,
            width: 256.0,
            height: 600.0,
        },
        Rectangle {
            x: 288.0,
            y: 120.0,
            width: 816.0,
            height: 600.0,
        },
        Rectangle {
            x: 24.0,
            y: 24.0,
            width: 1080.0,
            height: 72.0,
        },
        Rectangle {
            x: 48.0,
            y: 48.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            x: 184.0,
            y: 48.0,
            width: 120.0,
            height: 24.0,
        },
    ];
    let scroll_panel_view: Rectangle = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };
    let scroll_panel_offset: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let scroll_bound_offset: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    let mut test : Obj =Obj::create_empty_obj();

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("FEA solver, batteries lost in transit")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.gui_status_bar(layout_recs[2], Some(static_cstr("").as_c_str()));
        if d.gui_button(layout_recs[3], Some(static_cstr("File Open").as_c_str())) {
            let output = match nfd2::open_file_dialog(Some(".xlsx"), Some(&Path::new("."))).expect("oh no") {
                Response::Okay(file_path) => file_path,
                Response::OkayMultiple(_) => todo!(),
                Response::Cancel => todo!(),
            };
            test =
               fem_comp::create_obj_from_xlsx(output.to_str().unwrap()).expect("Create object error");
                           test.c_s();
        }
        if d.gui_button(layout_recs[4], Some(static_cstr("Quit").as_c_str())) {
            break;
        }
        if !test.is_empty() {
            d.draw_text(&format!("{test}"), 43, 130, 12, Color::BLACK)
        }
        d.gui_group_box(layout_recs[0], Some(static_cstr("Object info").as_c_str()));
        d.gui_scroll_panel(
            Rectangle {
                x: layout_recs[1].x,
                y: layout_recs[1].y,
                width: layout_recs[1].width - scroll_bound_offset.x,
                height: layout_recs[1].height - scroll_bound_offset.y,
            },
            scroll_panel_view,
            scroll_panel_offset,
        );

        d.clear_background(Color::WHITE);
    }
}
