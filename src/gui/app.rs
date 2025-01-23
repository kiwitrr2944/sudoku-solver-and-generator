use super::field_button::*;
use super::rule_button::*;
use crate::logic::board::Position;
use crate::logic::game;
use crate::logic::rules::{PermutationRule, RelationRule, Rule, SumRule};
use crate::logic::solver::Solver;
use crate::{choose_color, for_pos};
use gtk::glib::Propagation;
use gtk::prelude::{
    BoxExt, ButtonExt, DialogExt, EntryBufferExtManual, EntryExt, GridExt, GtkWindowExt,
    OrientableExt, PopoverExt, WidgetExt,
};
use relm4::factory::FactoryVecDeque;
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

const COLOR_LIST: [&str; 10] = [
    "white", "grey", "red", "green", "purple", "orange", "pink", "brown", "black", "yellow",
];

const N: usize = 9;
const R: usize = 3;
const C: usize = 3;

fn popup(text: &str) {
    let dialog = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Info,
        gtk::ButtonsType::Ok,
        text,
    );
    dialog.connect_response(|dialog, _| {
        dialog.close();
    });
    dialog.show();
}

pub struct App {
    fields: FactoryVecDeque<Field>,
    rules: FactoryVecDeque<RuleButton>,
    global_value: usize,
    rule_active: usize,
    game: game::Game,
    finished: usize,
    planning: bool,
    show_rules: bool,
    hints: bool,
}

#[derive(Debug)]
pub enum AppMsg {
    FieldClicked(usize),
    ChangeValue(usize),
    Solve,
    AddRule(usize, String),
    RuleActive(usize),
    Finished,
    TogglePlanning,
    ToggleRules,
    Help,
    Hints,
    Generate,
    Wrong,
    Load(String),
    Save(String),
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = usize;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("SUDOKU GAME by kiwitrr2944"),
            set_default_size: (500, 500),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 50,
                set_margin_all: 50,
                set_css_classes: &["white"],

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    #[name(value_label)]
                    gtk::Label {
                        #[watch]
                        set_label: &format!("Value: {}. Planning? {}. Click 'h' to show help", model.global_value, model.planning),
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        #[name(custom_rules)]
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,

                            gtk::Box {
                                gtk::MenuButton {
                                    set_label: "Add rule",
                                    #[watch]
                                    set_visible: model.planning,
                                    set_direction: gtk::ArrowType::Right,

                                    #[wrap(Some)]
                                    set_popover: popover = &gtk::Popover {
                                        set_position: gtk::PositionType::Right,

                                        gtk::Box {
                                            set_orientation: gtk::Orientation::Vertical,
                                            set_spacing: 5,

                                            gtk::Button {
                                                set_label: "Permutation",
                                                connect_clicked => AppMsg::AddRule(0, 0.to_string()),
                                            },

                                            gtk::Box {
                                                set_orientation: gtk::Orientation::Horizontal,
                                                gtk::Label {
                                                    set_label: "Sum: ",
                                                },
                                                gtk::Entry {
                                                    connect_activate[sender] => move |entry| {
                                                        let buffer = entry.buffer();
                                                        sender.input(AppMsg::AddRule(1, buffer.text().into()));
                                                        buffer.delete_text(0, None);
                                                    }
                                                },
                                            },

                                            gtk::Button {
                                                set_label: "Relation: ",
                                                connect_clicked => AppMsg::AddRule(2, 0.to_string()),
                                            }
                                        },
                                    },
                                },

                                #[local_ref]
                                rule_grid -> gtk::Grid {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_column_spacing: 15,
                                    set_row_spacing: 5,
                                }
                            }
                        },

                        #[local_ref]
                        field_grid -> gtk::Grid {
                            set_orientation: gtk::Orientation::Vertical,
                            set_column_spacing: 15,
                            set_row_spacing: 5,
                            #[watch]
                            set_css_classes: choose_color!(model.finished),
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            gtk::Label {
                                set_label: "Load from file: ",
                            },
                            gtk::Entry {
                                connect_activate[sender] => move |entry| {
                                    let buffer = entry.buffer();
                                    let path = buffer.text();
                                    buffer.delete_text(0, None);
                                    sender.input(AppMsg::Load(path.into()));
                                }
                            }
                        },
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            gtk::Label {
                                set_label: "Save game to file: ",
                            },
                            gtk::Entry {
                                connect_activate[sender] => move |entry| {
                                    let buffer = entry.buffer();
                                    let path = buffer.text();
                                    buffer.delete_text(0, None);
                                    sender.input(AppMsg::Save(path.into()));
                                }
                            }
                        }
                    },
                },

                gtk::Box {
                    gtk::Label {
                        #[watch]
                        set_visible: model.show_rules,
                        #[watch]
                        set_label: model.game.get_rules_state().as_str(),
                    }
                }
            },

            add_controller = gtk::EventControllerKey::new() {
                connect_key_pressed[sender] => move |_, keyval, _, _| {
                    if let Some(keyval) = keyval.to_unicode().and_then(|c| c.to_digit(36)) {
                        match keyval as usize {
                            0..=N => sender.input(AppMsg::ChangeValue(keyval as usize)),
                            15 => sender.input(AppMsg::Solve),
                            16 => sender.input(AppMsg::Generate),
                            17 => sender.input(AppMsg::Help),
                            25 => sender.input(AppMsg::Hints),
                            27 => sender.input(AppMsg::TogglePlanning),
                            31 => sender.input(AppMsg::ToggleRules),
                            _ => {}
                        }
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
                    FieldOutput::FieldClicked(index) => AppMsg::FieldClicked(index),
                });

        let rules =
            FactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), |msg| match msg {
                    RuleOutput::RuleClicked(index) => AppMsg::RuleActive(index),
                });

        let mut model = App {
            fields,
            rules,
            global_value: 0,
            rule_active: 0,
            game: game::Game::new(N, R, C),
            finished: 0,
            planning: true,
            show_rules: false,
            hints: false,
        };

        let field_grid = model.fields.widget();
        let rule_grid = model.rules.widget();
        let widgets = view_output!();

        for_pos!(N, |pos: Position| {
            let color = pos.default_color(R, C);
            model.fields.guard().push_back(color);
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        let fields_guard = self.fields.guard();
        let mut rules_guard = self.rules.guard();

        match msg {
            AppMsg::FieldClicked(index) => {
                if self.planning && rules_guard.len() > 0 {
                    if rules_guard.is_empty() {
                        return;
                    }
                    let pos = Position::from_index(index).unwrap();
                    let rule: Rule = self.game.get_rule(self.rule_active);

                    if rule.get_positions().contains(&pos) {
                        self.game.remove_position_from_rule(self.rule_active, pos);
                        fields_guard.send(index, FieldMsg::ChangeColor(pos.default_color(R, C)));
                    } else {
                        self.game.add_position_to_rule(self.rule_active, pos);
                        fields_guard.send(index, FieldMsg::ChangeColor(self.rule_active));
                    }
                } else {
                    self.game
                        .set_value(Position::from_index(index).unwrap(), self.global_value);
                    let state = self.game.check_rules();

                    if let (None, None) = state {
                        self.finished = 0;
                        sender.input(AppMsg::Finished);
                    }

                    fields_guard.send(index, FieldMsg::SetValue(self.global_value));
                }
            }
            AppMsg::ChangeValue(value) => {
                self.global_value = value;
            }

            AppMsg::Hints => {
                if self.hints {
                    for_pos!(N, |pos: Position| {
                        fields_guard.send(pos.index(), FieldMsg::SetValue(N + 1));
                    });
                    self.hints = false;
                } else {
                    let solver = Solver::new(self.game.clone(), false);

                    for_pos!(N, |pos: Position| {
                        let hints = solver.get_options(pos);
                        fields_guard.send(pos.index(), FieldMsg::SetHints(hints));
                    });
                    self.hints = true;
                }
            }

            AppMsg::Solve => {
                let mut sol = Solver::new(self.game.clone(), false);

                sol.solve();
                let solution = sol.get_solution();

                if let Some(solution) = solution {
                    for_pos!(N, |pos| {
                        let sval = solution.get_value(pos);
                        let curval = self.game.get_value(pos);
                        if curval == 0 {
                            self.game.set_value(pos, sval);
                            fields_guard.send(pos.index(), FieldMsg::SetValue(sval));
                        }
                    });
                    self.finished = 2;
                    sender.input(AppMsg::Finished);
                } else {
                    sender.input(AppMsg::Wrong);
                }
            }

            AppMsg::Finished => {
                self.finished = 3;
                popup("Finished");
            }

            AppMsg::TogglePlanning => {
                self.planning = !self.planning;
            }

            AppMsg::AddRule(t, value) => {
                let index = rules_guard.len();
                if index > 7 {
                    popup("Maximum number of rules reached");
                    return;
                }

                if t == 0 {
                    rules_guard.push_back((String::from("Permutation"), index));
                    self.game.add_rule(Rule::Permutation(PermutationRule::new(
                        vec![],
                        self.game.get_base_rule_count() + index,
                    )));
                } else if t == 1 {
                    let value = value.parse::<usize>();
                    if let Ok(value) = value {
                        rules_guard.push_back((format!("SUM: {}", value), index));
                        self.game.add_rule(Rule::Sum(SumRule::new(
                            vec![],
                            value,
                            self.game.get_base_rule_count() + index,
                        )));
                    } else {
                        popup("Invalid rule value");
                    }
                } else {
                    rules_guard.push_back((String::from("Relation"), index));
                    self.game.add_rule(Rule::Relation(RelationRule::new(
                        self.game.get_base_rule_count() + index,
                    )));
                }
            }

            AppMsg::RuleActive(index) => {
                let rule = self.game.get_rule(self.rule_active);

                for pos in rule.get_positions() {
                    let id = pos.index();
                    let color = pos.default_color(R, C);
                    fields_guard.send(id, FieldMsg::ChangeColor(color));
                }

                self.rule_active = index;

                let rule = self.game.get_rule(self.rule_active);
                for pos in rule.get_positions() {
                    let index = pos.index();
                    fields_guard.send(index, FieldMsg::ChangeColor(self.rule_active));
                }
            }

            AppMsg::Help => {
                popup(
                    "Help:\n\
                    press 0-N to choose current setting value,\n\
                    you can move with arrows to select field and press Enter to set value,\n\
                    clicking on field will also set current setting value,\n\
                    three types of rules are available: for each one select corresponding box from add rule, then select positions to apply,\n\
                    'f' to finish game using solver,\n\
                    'r' to toggle planning mode,\n\
                    'p' to show all possible values for every field,\n\
                    'g' to generate sudoku game,\n\
                ",
                );
            }

            AppMsg::Wrong => {
                self.finished = 2;
                popup("Board is not correct or rules are contradictory");
            }

            AppMsg::Generate => {
                self.finished = 0;
                let game = Solver::generate(self.game.clone());
                if let Some(game) = game {
                    self.game = game;
                    for_pos!(N, |pos: Position| {
                        let sval = self.game.get_value(pos);
                        self.game.set_value(pos, sval);
                        fields_guard.send(pos.index(), FieldMsg::SetValue(sval));
                    });
                } else {
                    sender.input(AppMsg::Wrong);
                }
            }

            AppMsg::ToggleRules => {
                self.show_rules = !self.show_rules;
            }

            AppMsg::Save(path) => {
                let result = self.game.save_to_file(&path);
                match result {
                    Ok(_) => {
                        popup("Game saved");
                    }
                    Err(e) => {
                        popup(&format!("Error saving game: {}", e));
                    }
                }
            }

            AppMsg::Load(path) => {
                self.finished = 0;
                let loaded = game::Game::load_from_file(&path);

                match loaded {
                    Ok(game) => {
                        self.game = game;
                    }
                    Err(e) => {
                        popup(&format!("Error loading game: {}", e));
                        return;
                    }
                }
                rules_guard.clear();

                for_pos!(N, |pos: Position| {
                    let sval = self.game.get_value(pos);
                    self.game.set_value(pos, sval);
                    fields_guard.send(pos.index(), FieldMsg::SetValue(sval));
                });

                for rule in self
                    .game
                    .rules()
                    .iter()
                    .skip(self.game.get_base_rule_count())
                {
                    let index = rules_guard.len();
                    rules_guard.push_back((
                        match rule {
                            Rule::Permutation(_) => "Permutation".to_string(),
                            Rule::Sum(sum) => format!("Sum: {}", sum.get_sum()),
                            Rule::Relation(_) => "Relation".to_string(),
                        },
                        index,
                    ));
                }
            }
        }
    }
}
