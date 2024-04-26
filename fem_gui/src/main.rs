use fem_comp::{self, vals::Obj};
use raylib::{ffi::Rectangle, prelude::*};

use nfd2::Response;

use std::{ffi::CString, path::Path};

fn static_cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}
#[derive(PartialEq)]
enum ConstrType {
    FixedC,
    FixedXoYNP,
    NonFixed,
}
// TODO Render the object usings lines , triangles,circles  and rectangles for different constraint types

fn determine(fl: &[f32; 3]) -> ConstrType {
    match fl {
        [x, y, z] if !(*x == 0.0 && *y == 0.0 && *z == 0.0) => ConstrType::FixedC,
        [x, y, z] if !(*x == 0.0 || *y == 0.0) && *z != 0.0 => ConstrType::FixedXoYNP,
        _ => ConstrType::NonFixed,
    }
}
fn main() {
    let (sc_h, sc_w) = (800, 1600);
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
    ];

    let mut test: Obj = Obj::create_empty_obj();

    let (mut rl, thread) = raylib::init()
        .size(sc_w, sc_h)
        .title("FEA solver, batteries lost in transit")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_rectangle_rec(layout_recs[1], Color::LIGHTGRAY);
        if !test.is_empty() {
            for element in test.elements.iter() {
                let (start_x, start_y, end_x, end_y) = (
                    test.nodes[element.b_id].0 + layout_recs[1].x,
                    test.nodes[element.b_id].1 + (layout_recs[1].height / 2.0) + layout_recs[0].y,
                    test.nodes[element.e_id].0 + layout_recs[1].x,
                    test.nodes[element.e_id].1 + (layout_recs[1].height / 2.0) + layout_recs[0].y,
                );

                for ele in test.constraints.iter() {
                    if ele.node_id == element.b_id {
                        if determine(&ele.stiffness) == ConstrType::FixedC {
                            d.draw_triangle(
                                Vector2 {
                                    x: 10.0 + start_x,
                                    y: 10.0 + start_y,
                                },
                                Vector2 {
                                    x: 5.0 + start_x,
                                    y: 5.0 + start_y,
                                },
                                Vector2 {
                                    x: 5.0 + start_x,
                                    y: 5.0 + start_y,
                                },
                                Color::BLACK,
                            );
                        } else if determine(&ele.stiffness) == ConstrType::FixedXoYNP {
                            d.draw_circle(start_x as i32, start_y as i32, 10.0, Color::BLACK);
                        } else if determine(&ele.stiffness) == ConstrType::NonFixed {
                            d.draw_rectangle(start_x as i32, start_y as i32, 10, 10, Color::BLACK);
                        }
                    }  if ele.node_id == element.e_id {
                        if determine(&ele.stiffness) == ConstrType::FixedC {
                            d.draw_triangle(
                                Vector2 {
                                    x: 10.0 + end_x,
                                    y: 10.0 + end_y,
                                },
                                Vector2 {
                                    x: 5.0 + end_x,
                                    y: 5.0 + end_y,
                                },
                                Vector2 {
                                    x: 5.0 + end_x,
                                    y: 5.0 + end_y,
                                },
                                Color::BLACK,
                            );
                        } else if determine(&ele.stiffness) == ConstrType::FixedXoYNP {
                            d.draw_circle(end_x as i32, end_y as i32, 10.0, Color::BLACK);
                        } else if determine(&ele.stiffness) == ConstrType::NonFixed {
                            d.draw_rectangle(end_x as i32, end_y as i32, 10, 10, Color::BLACK);
                        }
                    }
                }

                d.draw_line(
                    (start_x) as i32,
                    (start_y) as i32,
                    (end_x) as i32,
                    (end_y) as i32,
                    Color::BLACK,
                );
            }
        }

        d.gui_status_bar(layout_recs[2], Some(static_cstr("").as_c_str()));
        if d.gui_button(layout_recs[3], Some(static_cstr("Input File").as_c_str())) {
            let output =
                match nfd2::open_file_dialog(Some("xlsx"), Some(&Path::new("."))).expect("oh no") {
                    Response::Okay(file_path) => file_path,
                    Response::OkayMultiple(_) => todo!(),
                    Response::Cancel => todo!(),
                };
            test = fem_comp::create_obj_from_xlsx(output.to_str().unwrap())
                .expect("Create object error");
            test.c_s();
        }
        if d.gui_button(layout_recs[4], Some(static_cstr("Quit").as_c_str())) {
            break;
        }
        if !test.is_empty() {
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
