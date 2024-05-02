use fem_comp::{self, vals::Obj};
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
    FixedXoYNP,
    NonFixed,
}
// TODO Render the object usings lines , triangles,circles  and rectangles for different constraint types
impl ConstrType {
    fn determine(fl: &[f32; 3]) -> Self {
        match fl {
            [x, y, z] if !(*x == 0.0 && *y == 0.0 && *z == 0.0) => ConstrType::FixedC,
            [x, y, z] if !(*x == 0.0 || *y == 0.0) && *z != 0.0 => ConstrType::FixedXoYNP,
            _ => ConstrType::NonFixed,
        }
    }
    pub fn draw_determined(fl: &[f32; 3],x : f32,y : f32 ,d : &mut RaylibDrawHandle) {
        match ConstrType::determine(fl) {
            // v1
            // v2 v3
            ConstrType::FixedC => d.draw_triangle(
                Vector2 {
                    x: x,
                    y: y - 4.0,
                },
                Vector2 {
                    x: x - 6.0,
                    y: 3.0 + y,
                },
                Vector2 {
                    x: 6.0 + x,
                    y: 3.0 + y,
                },
                Color::BLACK,
            ),
            ConstrType::FixedXoYNP => {
                d.draw_circle(x as i32, y as i32, 10.0, Color::BLACK)
            }
            _ => d.draw_rectangle(x as i32, y as i32, 10, 10, Color::BLACK),
        }
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
        d.gui_status_bar(layout_recs[2], Some(static_cstr("").as_c_str()));
        if d.gui_button(layout_recs[3], Some(static_cstr("Input File").as_c_str())) {
            let output = match nfd::open_file_dialog(Some("xlsx"), Some(".")).expect("oh no") {
                Response::Okay(file_path) => file_path,
                Response::OkayMultiple(_) => todo!(),
                Response::Cancel => todo!(),
            };
            test = fem_comp::create_obj_from_xlsx(output.as_str()).expect("Create object error");
            test.c_s();
        }
        if d.gui_button(layout_recs[4], Some(static_cstr("Quit").as_c_str())) {
            break;
        }
        if !test.is_empty() {
            for element in test.elements.iter() {
                let (start_x, start_y, end_x, end_y) = (
                    test.nodes[element.b_id].0 + layout_recs[1].x,
                    test.nodes[element.b_id].1 + (layout_recs[1].height / 2.0) + layout_recs[0].y,
                    test.nodes[element.e_id].0 + layout_recs[1].x,
                    test.nodes[element.e_id].1 + (layout_recs[1].height / 2.0) + layout_recs[0].y,
                );

                d.draw_line(
                    (start_x) as i32,
                    (start_y) as i32,
                    (end_x) as i32,
                    (end_y) as i32,
                    Color::BLACK,
                );

                for ele in test.constraints.iter() {
                    if ele.node_id == element.b_id {
                        ConstrType::draw_determined(&ele.stiffness,start_x,start_y ,&mut d) ;
                    }
                    if ele.node_id == element.e_id {
                        ConstrType::draw_determined(&ele.stiffness,end_x,end_y ,&mut d) ;    
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
        }
        else {
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
