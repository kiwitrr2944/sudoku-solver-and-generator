use super::field::{Field, FieldMsg, FieldOutput};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt, *};
use crate::logic::game;
use relm4::factory::FactoryVecDeque;
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

const N: usize = 6;
const R: usize = 2;
const C: usize = 3;
const COLOR_LIST : [&str; 10] = ["white", "grey", "red", "green", "purple", "orange", "pink", "brown", "black", "yellow"];

macro_rules! choose_color {
    ($color_index:expr) => {
        &[&COLOR_LIST[$color_index]]
    };
}

pub struct App {
    fields: FactoryVecDeque<Field>,
    global_value: usize,
    game: game::Game,
    finished: usize,
}

#[derive(Debug)]
pub enum AppMsg {
    RequestValue(usize),
    ChangeValue(usize),
    Solve,
    Finished,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = usize;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("SUDOKU GAME by kiwitrr2944"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 50,
                set_margin_all: 50,
                
                #[name(value_label)]
                gtk::Label {
                    #[watch]
                    set_label: &format!("Current setting value: {}", model.global_value),
                    set_css_classes: choose_color!(0),
                },
                
                #[local_ref]
                field_grid -> gtk::Grid {
                    set_orientation: gtk::Orientation::Vertical,
                    set_column_spacing: 15,
                    set_row_spacing: 5,
                    #[watch]
                    set_css_classes: choose_color!(model.finished),
                }
            },

            add_controller = gtk::EventControllerKey::new() {
                connect_key_pressed[sender] => move |_, keyval, _, _| {
                    let keyval = keyval.to_unicode().unwrap_or_default().to_digit(36);
                    dbg!(keyval);
                    match keyval {
                        Some(keyval) => {
                            match keyval as usize {
                                0..=N => sender.input(AppMsg::ChangeValue(keyval as usize)),
                                15 => {sender.input(AppMsg::Solve);},
                                _ => {}
                            }
                        },
                        None => {}
                    }
                    Propagation::Proceed
                },
            }
        }
    }

    fn init(
        _field: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let fields =
            FactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), |msg| match msg {
                    FieldOutput::RequestValue(index) => AppMsg::RequestValue(index),
                });

                
        let mut model = App {
            fields,
            global_value: 0,
            game: crate::logic::game::Game::new(N, R, C),
            finished: 0,
        };

        let field_grid = model.fields.widget();
        let widgets = view_output!();

        // Initialize the grid with n x n fields
        // You can change this value to the desired grid size
        for i in 0..N {
            for j in 0..N {
                let color = (i / C + j / R) % 2;
                model.fields.guard().push_back(color as usize);
            }
        }

        ComponentParts { model, widgets }
    }


    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        let fields_guard = self.fields.guard();

        match msg {
            AppMsg::RequestValue(index) => {
                self.game.set_value(crate::logic::board::Position::new(1 + index%N, 1 + index/N), self.global_value);
                let state = self.game.check_rules();
                match state {
                    (None, None) => {
                        self.finished = 3;
                        sender.input(AppMsg::Finished);
                    }
                    _ => {}
                }
                fields_guard.send(index, FieldMsg::SetValue(self.global_value));
            },
            AppMsg::ChangeValue(value) => {
                self.global_value = value;
            },
            AppMsg::Solve => {
                let mut sol = crate::logic::solver::Solver::new(self.game.clone());
                sol.board.display();
                sol.solve();
                let solution = sol.get_solution();

                if let Some(solution) = solution {
                    for i in 0..N {
                        for j in 0..N {
                            let index = i * N + j;
                            let pos = crate::logic::board::Position::new(1 + j, 1 + i);
                            let sval = solution.get_value(pos).unwrap();
                            let curval = self.game.get_value(pos);
                            if curval == None {
                                self.game.set_value(pos, sval);
                                fields_guard.send(index, FieldMsg::SetValue(sval));
                            }
                        }
                    }
                    self.finished = 3;
                } else {
                    self.finished = 2;
                    dbg!("No solution found");
                }
                sender.input(AppMsg::Finished);
            },
            AppMsg::Finished => {
                dbg!("Finished");
            },
        }
    }
}