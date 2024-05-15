use std::ffi::CString;

use fem_comp::vals::Obj;
extern crate nfd;
extern crate raylib;
use nfd::Response;
use raylib::{consts::KeyboardKey::*, prelude::*};

// TODO
// 1. Zoom view  [X]
// 2. Rich text output []
// 3. Size coefficent []

#[derive(PartialEq, Clone)]
enum ConstrType {
    FixedC,
    FixedXaPoYaP,
    NonFixed,
}
#[derive(Clone)]
struct VisNode {
    pos: Vector2,
    constraint: ConstrType,
}
struct World {
    nodes: Vec<VisNode>,
}

impl ConstrType {
    fn determine(fl: [f32; 3]) -> Self {
        match fl {
            [x, y, p] if !(x == 0.0 && y == 0.0 && p == 0.0) => ConstrType::FixedC,
            [x, y, p] if !(x == 0.0 || y == 0.0) && p == 0.0 => ConstrType::FixedXaPoYaP,
            _ => ConstrType::NonFixed,
        }
    }
}

impl World {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    fn init_world(&mut self, object: &Obj, offset: &Rectangle) {
        let constraints = &object.constraints;

        for (node_id, i) in object.nodes.iter().enumerate() {
            if constraints.iter().any(|f| f.node_id == node_id) {
                self.nodes.push(VisNode {
                    pos: Vector2 {
                        x: i.0 + offset.x + 10.0,
                        y: i.1 + (offset.height / 1.2) + 100.0,
                    },
                    constraint: ConstrType::determine(
                        constraints[constraints
                            .iter()
                            .position(|f| f.node_id == node_id)
                            .unwrap()]
                        .stiffness,
                    ),
                })
            } else {
                self.nodes.push(VisNode {
                    pos: Vector2 {
                        x: i.0 + offset.x + 10.0,
                        y: i.1 + (offset.height / 1.2) + 100.0,
                    },
                    constraint: ConstrType::NonFixed,
                })
            }
        }
    }
    fn new_node(&mut self, pos: &Vector2, ct: ConstrType) {
        self.nodes.push(VisNode {
            pos: Vector2 { x: pos.x, y: pos.y },
            constraint: ct,
        })
    }
    // fn new_elements(&mut self,pos)
    fn remove_node(&mut self, pos: &Vector2) {
        match self
            .nodes
            .iter()
            .position(|f| f.pos.distance_to(*pos) <= 10.0)
        {
            Some(x) => self.nodes.remove(x),
            None => VisNode { pos: Vector2{x: 0.0,y:0.0}, constraint: ConstrType::NonFixed},
        };
    }
    fn draw_nodes(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        self.nodes.iter().for_each(|f| match &f.constraint {
            // v1
            // v2 v3
            ConstrType::FixedC => d.draw_triangle(
                Vector2 {
                    x: f.pos.x,
                    y: f.pos.y - 5.0,
                },
                Vector2 {
                    x: f.pos.x - 5.0,
                    y: 5.0 + f.pos.y,
                },
                Vector2 {
                    x: 5.0 + f.pos.x,
                    y: 5.0 + f.pos.y,
                },
                Color::BLACK,
            ),
            ConstrType::FixedXaPoYaP => {
                d.draw_circle(f.pos.x as i32, f.pos.y as i32, 10.0, Color::BLACK)
            }
            ConstrType::NonFixed => {
                d.draw_rectangle(f.pos.x as i32, (f.pos.y as i32) - 5, 10, 10, Color::BLACK)
            }
        })
    }
    fn draw_elements(&self, d: &mut RaylibMode2D<RaylibDrawHandle>) {
        if self.nodes.len() > 1 {
            for (x, _) in self.nodes.iter().step_by(2).enumerate() {
                d.draw_line_v(self.nodes[x].pos, self.nodes[x + 1].pos, Color::BLACK);
            }
            
        }
    }
}

pub fn static_cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn main() {
    let mut file_name: String = "".to_string();
    let mut test: Obj = Obj::create_empty_obj();
    let mut world = World::new();

    let (mut sc_h, mut sc_w) = (1000, 1200);
    let mut layout_recs = [
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
        Rectangle {
            // View additional info
            x: 300.0,
            y: 100.0,
            width: 300.0,
            height: 200.0,
        },
        Rectangle {
            // Zoom button
            x: 740.0,
            y: 20.0,
            width: 120.0,
            height: 24.0,
        },
        Rectangle {
            // Window
            x: 20.0,
            y: 100.0,
            width: 200.0,
            height: (sc_h - 105) as f32,
        },
    ];

    let (mut rl, thread) = raylib::init()
        .size(sc_w, sc_h)
        .title("FEA solver, batteries lost in transit")
        .resizable()
        .build();

    rl.set_window_min_size(800, 1000);
    rl.set_target_fps(120);
    let mut obj_open = false;

    let mut camra = Camera2D {
        offset: Vector2::new(320.0, (sc_h - 15) as f32).into(),
        target: Vector2::new(320.0, (sc_h - 15) as f32).into(),
        rotation: 0.0,
        zoom: 1.0,
    };

    while !rl.window_should_close() {
        camra.zoom += rl.get_mouse_wheel_move() as f32 * 0.05;
        camra.zoom = camra.zoom.max(0.25).min(1.5);

        layout_recs[0].height = (sc_h - 105) as f32;
        layout_recs[1].width = (sc_w - 325) as f32;
        layout_recs[1].height = (sc_h - 105) as f32;
        layout_recs[2].width = sc_w as f32;

        camra.target = Vector2 {
            x: 320.0,
            y: (sc_h - 15) as f32,
        };

        camra.offset = camra.target;

        let mut d = rl.begin_drawing(&thread);
        (sc_h, sc_w) = (d.get_screen_height(), d.get_screen_width());
        d.clear_background(Color::WHITE);
        d.draw_rectangle_rec(layout_recs[1], Color::LIGHTGRAY);

        if !test.is_empty() {
            {
                let mut d = d.begin_mode2D(camra);
                let m_pos = d.get_mouse_position();
                world.draw_elements(&mut d);
                world.draw_nodes(&mut d);
                if layout_recs[1].check_collision_point_rec(m_pos) {
                    if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
                        world.remove_node(&m_pos);
                    }
                    if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                        world.new_node(&m_pos, ConstrType::NonFixed);
                    }
                }
            }
        } else {
            d.draw_text(
                &format!("No object"),
                (layout_recs[0].x as i32) + 10,
                (layout_recs[0].y as i32) + 15,
                11,
                Color::BLACK,
            );
        }

        d.gui_status_bar(layout_recs[2], Some(static_cstr("").as_c_str()));

        // Bunch of buttons
        if d.is_key_pressed(KEY_R)
            || d.gui_button(
                layout_recs[8],
                Some(static_cstr(&format!("Zoom : {}", camra.zoom)).as_c_str()),
            )
        {
            camra.zoom = 1.0;
        }
        if d.gui_button(layout_recs[3], Some(static_cstr("Input File").as_c_str())) {
            file_name = match nfd::open_file_dialog(Some("xlsx"), None).expect("oh no") {
                Response::Okay(file_path) => file_path,
                _ => continue,
    
            };
            test = fem_comp::create_obj_from_xlsx(file_name.as_str()).expect("Create object error");
            test.c_s();
            world.init_world(&test, &layout_recs[1]);
            obj_open = true;
        }

        if d.gui_button(layout_recs[4], Some(static_cstr("Quit").as_c_str())) {
            break;
        }
        if obj_open {
            if d.gui_button(layout_recs[5], Some(static_cstr("Clear Input").as_c_str())) {
                test = Obj::create_empty_obj();
                world = World::new();
                obj_open = false;
            }
            if d.gui_button(layout_recs[6], Some(static_cstr("Save object").as_c_str())) && !(test.is_empty()) {
                test.write_data(file_name.as_str()).unwrap();
            }

            if d.gui_button(
                Rectangle {
                    x: layout_recs[0].x + 15.0,
                    y: layout_recs[0].y + 15 as f32,
                    width: 120.0,
                    height: 24.0,
                },
                Some(static_cstr("Nodes").as_c_str()),
            ) {
                d.gui_group_box(layout_recs[9], Some(static_cstr("Nodes").as_c_str()));

                d.draw_text(
                    &format!("{:?}", test.nodes),
                    (layout_recs[9].x as i32) + 10,
                    (layout_recs[9].y as i32) + 15,
                    10,
                    Color::BLACK,
                );
            }
            if d.gui_button(
                Rectangle {
                    x: layout_recs[0].x + 15.0,
                    y: layout_recs[0].y + 45 as f32,
                    width: 120.0,
                    height: 24.0,
                },
                Some(static_cstr("Loads").as_c_str()),
            ) {
                d.gui_group_box(layout_recs[9], Some(static_cstr("Loads").as_c_str()));
                d.draw_text(
                    &format!("{:?}", test.loads),
                    (layout_recs[9].x as i32) + 10,
                    (layout_recs[9].y as i32) + 15,
                    10,
                    Color::BLACK,
                );
            }
            if d.gui_button(
                Rectangle {
                    x: layout_recs[0].x + 15.0,
                    y: layout_recs[0].y + 75 as f32,
                    width: 120.0,
                    height: 24.0,
                },
                Some(static_cstr("PhysGeos").as_c_str()),
            ) {
                d.gui_group_box(layout_recs[9], Some(static_cstr("PhysGeos").as_c_str()));
                d.draw_text(
                    &format!("{:?}", test.physgeos),
                    (layout_recs[9].x as i32) + 10,
                    (layout_recs[9].y as i32) + 15,
                    10,
                    Color::BLACK,
                );
            }
            if d.gui_button(
                Rectangle {
                    x: layout_recs[0].x + 15.0,
                    y: layout_recs[0].y + 105 as f32,
                    width: 120.0,
                    height: 24.0,
                },
                Some(static_cstr("Constraints").as_c_str()),
            ) {
                d.gui_group_box(layout_recs[9], Some(static_cstr("Constraints").as_c_str()));
                d.draw_text(
                    &format!("{:?}", test.constraints),
                    (layout_recs[9].x as i32) + 10,
                    (layout_recs[9].y as i32) + 15,
                    10,
                    Color::BLACK,
                );
            }
            if d.gui_button(
                Rectangle {
                    x: layout_recs[0].x + 15.0,
                    y: layout_recs[0].y + 135 as f32,
                    width: 120.0,
                    height: 24.0,
                },
                Some(static_cstr("S vec").as_c_str()),
            ) {
                d.gui_group_box(layout_recs[9], Some(static_cstr("S vec").as_c_str()));
                d.draw_text(
                    &format!("{:?}", test.s),
                    (layout_recs[9].x as i32) + 10,
                    (layout_recs[9].y as i32) + 15,
                    10,
                    Color::BLACK,
                );
            }
        }
        else {
            if d.gui_button(layout_recs[5], Some(static_cstr("New object").as_c_str())) {
                test = Obj::create_empty_obj();
                world = World::new();
                obj_open = true;
            }
        }
        d.gui_group_box(
            layout_recs[0],
            Some(static_cstr(&format!("Object : {}", file_name)).as_c_str()),
        );
    }
}
