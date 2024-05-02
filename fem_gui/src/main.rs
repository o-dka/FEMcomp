use fem_comp::vals::Obj;
extern crate nfd;
extern crate raylib;
use raylib::{ffi::Rectangle, prelude::*};

use nfd::Response;

use std::ffi::CString;

fn static_cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

#[derive(PartialEq)]
enum ConstrType {
    FixedC,
    FixedXaPoYaP,
    NonFixed,
    // Err
}
// TODO Render the object usings lines , triangles,circles  and rectangles for different constraint types
impl ConstrType {
    fn determine(fl: [f32; 3]) -> Self {
        // Fix logic issues
        match fl {
            [x, y, p] if !(x == 0.0 && y == 0.0 && p == 0.0) => ConstrType::FixedC,
            [x, y, p] if !(x == 0.0 && p == 0.0) || !(y == 0.0 && p == 0.0) => {
                ConstrType::FixedXaPoYaP
            }
            _ => ConstrType::NonFixed,
        }
    }
    pub fn draw_determined(fl: &[f32; 3], x: f32, y: f32, d: &mut RaylibDrawHandle) {
        match ConstrType::determine(*fl) {
            // v1
            // v2 v3
            ConstrType::FixedC => d.draw_triangle(
                Vector2 { x: x, y: y - 8.0 },
                Vector2 {
                    x: x - 8.0,
                    y: 8.0 + y,
                },
                Vector2 {
                    x: 8.0 + x,
                    y: 8.0 + y,
                },
                Color::BLACK,
            ),
            ConstrType::FixedXaPoYaP => d.draw_circle(x as i32, y as i32, 10.0, Color::BLACK),
            ConstrType::NonFixed => {
                d.draw_rectangle(x as i32, (y as i32) - 5, 10, 10, Color::BLACK)
            }
        }
    }
}

fn main() {
    let (sc_h, sc_w) = (800, 1000);
    let _zoom: f32 = 1.0;
    let layout_recs = [
        Rectangle {
            // Object info box
            x: 0.0,
            y: 100.0,
            width: 300.0,
            height: (sc_h - 105) as f32,
        },
        Rectangle {
            // Visual output
            x: 320.0,
            y: 100.0,
            width: (sc_w - 325) as f32,
            height: (sc_h - 105) as f32,
        },
        Rectangle {
            // Status bar
            x: 0.0,
            y: 0.0,
            width: sc_w as f32,
            height: 65.0,
        },
        Rectangle {
            // Input File button
            x: 20.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            // Quit button
            x: 160.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            // Clear button
            x: 300.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            // Save button
            x: 440.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
    ];
    let mut test: Obj = Obj::create_empty_obj();

    let (mut rl, thread) = raylib::init()
        .size(sc_w, sc_h)
        .title("FEA solver, batteries lost in transit")
        .build();
    let mut obj_open = false;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let _mouse = d.get_mouse_position();

        d.draw_rectangle_rec(layout_recs[1], Color::LIGHTGRAY);
        d.gui_status_bar(layout_recs[2], Some(static_cstr("").as_c_str()));

        if d.gui_button(layout_recs[3], Some(static_cstr("Input File").as_c_str())) {
            let output = match nfd::open_file_dialog(Some("xlsx"), Some(".")).expect("oh no") {
                Response::Okay(file_path) => file_path,
                _ => continue,
            };
            test = fem_comp::create_obj_from_xlsx(output.as_str()).expect("Create object error");
            test.c_s();
            obj_open = true;
        }

        if d.gui_button(layout_recs[4], Some(static_cstr("Quit").as_c_str())) {
            break;
        }

        if obj_open {
            if d.gui_button(layout_recs[5], Some(static_cstr("Clear Input").as_c_str())) {
                test = Obj::create_empty_obj();
                obj_open = false;
            }
            if d.gui_button(layout_recs[6], Some(static_cstr("Save object").as_c_str())) {
               test.write_data().unwrap();
            }
        }

        if !test.is_empty() {
            for element in test.elements.iter() {
                let (start, end) = (element.b_id, element.e_id);
                let (start_x, start_y, end_x, end_y) = (
                    test.nodes[start].0 + layout_recs[1].x + 10.0,
                    test.nodes[start].1 + (layout_recs[1].height / 2.0) + layout_recs[0].y - 10.0,
                    test.nodes[end].0 + layout_recs[1].x + 10.0,
                    test.nodes[end].1 + (layout_recs[1].height / 2.0) + layout_recs[0].y - 10.0,
                );

                d.draw_line(
                    (start_x) as i32,
                    (start_y) as i32,
                    (end_x) as i32,
                    (end_y) as i32,
                    Color::BLACK,
                );

                for ele in test.constraints.iter() {
                    // Make this a pattern
                    if ele.node_id == start {
                        ConstrType::draw_determined(&ele.stiffness, start_x, start_y, &mut d);
                        continue;
                    } else if ele.node_id == end {
                        ConstrType::draw_determined(&ele.stiffness, end_x, end_y, &mut d);
                        continue;
                    } else if test.constraints.iter().any(|f| f.node_id == start) {
                        ConstrType::draw_determined(&[0.0, 0.0, 0.0], end_x, end_y, &mut d);
                    }
                }
            }
            d.draw_text(
                &format!("{test}"),
                (layout_recs[0].x as i32) + 10,
                (layout_recs[0].y as i32) + 15,
                10,
                Color::BLACK,
            );
        } else {
            d.draw_text(
                &format!("No object"),
                (layout_recs[0].x as i32) + 10,
                (layout_recs[0].y as i32) + 15,
                11,
                Color::BLACK,
            );
        }
        d.gui_group_box(layout_recs[0], Some(static_cstr("Object info").as_c_str()));
        d.clear_background(Color::WHITE);
    }
}
