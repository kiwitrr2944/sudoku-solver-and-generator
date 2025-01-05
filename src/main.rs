use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, ButtonExt, GridExt, GtkWindowExt, OrientableExt, WidgetExt};
use logic::game;
use relm4::factory::positions::GridPosition;
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque, Position};
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

mod gui;
mod logic;

const N: usize = 4;
const R: usize = 2;
const C: usize = 2;

const COLOR_LIST : [&str; 10] = ["white", "grey", "red", "green", "purple", "orange", "pink", "brown", "black", "yellow"];

macro_rules! choose_color {
    ($color_index:expr) => {
        &[&COLOR_LIST[$color_index]]
    };
}

#[derive(Debug)]
struct Field {
    value: usize,
    display_value: String,
    color: usize,
    index: usize,
}

#[derive(Debug)]
enum FieldMsg {
    ChangeValue,
    SetValue(usize),
}

#[derive(Debug)]
enum FieldOutput {
    RequestValue(usize),
}

struct FieldWidgets {
    label: gtk::Button,
}

impl Position<GridPosition, DynamicIndex> for Field {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index();
        let x = index % N;
        let y = index / N;
        GridPosition {
            column: y as i32,
            row: x as i32,
            width: 1,
            height: 1,
        }
    }
}

impl FactoryComponent for Field {
    type Init = usize;
    type Input = FieldMsg;
    type Output = FieldOutput;
    type CommandOutput = ();
    type Root = gtk::Box;
    type Widgets = FieldWidgets;
    type ParentWidget = gtk::Grid;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        relm4::view! {
            root = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
            }
        }
        root
    }

    fn init_model(color: Self::Init, index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let index = index.current_index();
        
        Self { value: 0, display_value: String::from("_"), color, index}
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &gtk::Widget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        relm4::view! {
            #[local_ref]
            root -> gtk::Box {
                #[name(label)]
                gtk::Button {
                    set_css_classes: choose_color!(self.color),
                    set_label: &self.display_value,
                    set_size_request: (100, 50), // Set the size of the button
                    connect_clicked => FieldMsg::ChangeValue,
                    set_margin_all: 5,
                }
            }
        }

        FieldWidgets { label }
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            FieldMsg::ChangeValue => {
                sender.output(FieldOutput::RequestValue(self.index)).unwrap();
            }
            FieldMsg::SetValue(value) => {
                self.value = value;
                self.display_value = match value {
                    0 => String::from("_"),
                    _ => value.to_string(),
                };
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: FactorySender<Self>) {
        widgets.label.set_label(&self.display_value);
        widgets.label.set_css_classes(choose_color!(self.color));
    }
}

struct App {
    fields: FactoryVecDeque<Field>,
    global_value: usize,
    game: game::Game,
    finished: usize,
}

#[derive(Debug)]
enum AppMsg {
    RequestValue(usize),
    ChangeValue(usize),
    Solve,
    Finished,
}

#[relm4::component]
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
            game: logic::game::Game::new(N, R, C),
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
                self.game.set_value(logic::board::Position::new(1 + index%N, 1 + index/N), self.global_value);
                let state = self.game.check_rules();
                match state {
                    (None, None) => {
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
                let mut S = logic::solver::Solver::new(self.game.clone());
                S.solve();
                S.display_solutions();
                let solution = S.get_solution();
                if let Some(solution) = solution {
                    for i in 0..N {
                        for j in 0..N {
                            let index = i * N + j;
                            let pos = logic::board::Position::new(1 + j, 1 + i);
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

fn main() {
    let app = RelmApp::new("relm4.sudoku");
    relm4::set_global_css(
        ".green { background: #00ad5c; } \
         .grey { background: #DDDDDD; font-size: 20px; } \
         .blue { background: #33EEFF; } \
         .red { background: #FF5500; } \
         .yellow { background: #FFFF00; } \
         .purple { background: #800088; } \
         .orange { background: #FFA500; } \
         .pink { background: #FFC0CB; } \
         .brown { background:rgb(187, 83, 83); } \
         .black { background: #000000; } \
         .white { background: #FFFFFF; font-size: 20px; }",
    );

    app.run::<App>(0);
}
