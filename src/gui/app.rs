use super::field_button::{Field, FieldMsg, FieldOutput};
use super::rule_button::{RuleButton, RuleOutput};
use gtk::glib::Propagation;
use gtk::prelude::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt, *};
use crate::logic::game;
use crate::logic::rules::{PermutationRule, Rule};
use relm4::factory::FactoryVecDeque;
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[warn(unknown_lints, reason="CHANGEDIMENSION")]
const N: usize = 6;
const R: usize = 2;
const C: usize = 3;
const COLOR_LIST : [&str; 10] = ["white", "grey", "red", "green", "purple", "orange", "pink", "brown", "black", "yellow"];

macro_rules! choose_color {
    ($color_index:expr) => {
        &[&COLOR_LIST[$color_index]]
    };
}

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
}

#[derive(Debug)]
pub enum AppMsg {
    FieldClicked(usize),
    ChangeValue(usize),
    Solve,
    AddRule,
    RuleActive(usize),
    Finished,
    TogglePlanning,
    ToggleRules,
    Help,
    Generate,
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
                set_css_classes: choose_color!(0),
                
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    #[name(value_label)]
                    gtk::Label {
                        #[watch]
                        set_label: &format!("Current setting value: {}", model.global_value),
                    },
                    
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        #[name(custom_rules)]
                        gtk::Box {
                            #[watch]
                            set_orientation: gtk::Orientation::Horizontal,
                            
                            gtk::Button {
                                #[watch]
                                set_visible: model.planning,
                                set_label: "Add Rule",
                                connect_clicked => AppMsg::AddRule,
                            },
                            
                            #[local_ref]
                            rule_grid -> gtk::Grid {
                                set_orientation: gtk::Orientation::Vertical,
                                set_column_spacing: 15,
                                set_row_spacing: 5,
                            }
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
                },
                    
                gtk::Box {
                    gtk::Label {
                        #[watch]
                        set_visible: model.show_rules,
                        #[watch]
                        set_label: format!("{}", model.game.get_rules_state()).as_str(),
                    }
                }
            },

            add_controller = gtk::EventControllerKey::new() {
                connect_key_pressed[sender] => move |_, keyval, _, _| {
                    if let Some(keyval) = keyval.to_unicode().and_then(|c| c.to_digit(36)) {
                        dbg!(keyval);
                        match keyval as usize {
                            0..=N => sender.input(AppMsg::ChangeValue(keyval as usize)),
                            15 => sender.input(AppMsg::Solve),
                            16 => sender.input(AppMsg::Generate),
                            17 => sender.input(AppMsg::Help),
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

        let rules = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |msg| match msg {
                RuleOutput::RuleClicked(index) => AppMsg::RuleActive(index),
            });

                
        let mut model = App {
            fields,
            rules,
            global_value: 0,
            rule_active: 0,
            game: crate::logic::game::Game::new(N, R, C),
            finished: 0,
            planning: true,
            show_rules: false,
        };


        let field_grid = model.fields.widget();
        let rule_grid = model.rules.widget();
        let widgets = view_output!();

        for i in 0..N {
            for j in 0..N {
                let color = (i / C + j / R) % 2;
                model.fields.guard().push_back(8 + color);
            }
        }

        ComponentParts { model, widgets }
    }


    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        let fields_guard = self.fields.guard();
        let mut rules_guard = self.rules.guard();

        match msg {
            AppMsg::FieldClicked(index) => {
                dbg!("Field clicked", index, self.planning);
                if self.planning {
                    if rules_guard.is_empty() {
                        return ;
                    }
                    let pos = crate::logic::board::Position::new(1 + index%N, 1 + index/N).unwrap();
                    let rule: Rule = self.game.get_rule(self.rule_active);
                    
                    if rule.get_positions().contains(&pos) {
                        dbg!("Remove position", pos);
                    }
                    else {
                        self.game.add_position_to_rule(self.rule_active, pos);
                        fields_guard.send(index, FieldMsg::ChangeColor(self.rule_active));
                        dbg!("Add position", pos);
                    }
                }
                else {
                    self.game.set_value(crate::logic::board::Position::new(1 + index%N, 1 + index/N), self.global_value);
                    let state = self.game.check_rules();
                    match state {
                        (None, None) => {
                            self.finished = 0;
                            sender.input(AppMsg::Finished);
                        }
                        _ => {}
                    }
                    fields_guard.send(index, FieldMsg::SetValue(self.global_value));
                }
            },
            AppMsg::ChangeValue(value) => {
                self.global_value = value;
            },
            
            AppMsg::Solve => {
                let mut sol = crate::logic::solver::Solver::new(self.game.clone(), false);

                sol.solve();
                let solution = sol.get_solution();

                if let Some(solution) = solution {
                    for i in 0..N {
                        for j in 0..N {
                            let index = i * N + j;
                            let pos = crate::logic::board::Position::new(1 + j, 1 + i);
                            let sval = solution.get_value(pos);
                            let curval = self.game.get_value(pos);
                            if curval == 0 {
                                self.game.set_value(pos, sval);
                                fields_guard.send(index, FieldMsg::SetValue(sval));
                            }
                        }
                    }
                    self.finished = 0;
                } else {
                    self.finished = 1;
                    dbg!("No solution found");
                }
                sender.input(AppMsg::Finished);
            },

            AppMsg::Finished => {
                dbg!("Finished");
            },
            
            AppMsg::TogglePlanning => {
                self.planning = !self.planning;
                dbg!(self.game.rules());
            },

            AppMsg::AddRule => {
                dbg!("Add rule");
                let index = rules_guard.len();
                if index > 9 {
                    popup("Maximum number of rules reached");
                    return;
                }
                rules_guard.push_back((String::from("RULE"), index));
                self.game.add_rule(Rule::Permutation(PermutationRule::new(vec![], self.game.get_base_rule_count() + index)));
            },

            AppMsg::RuleActive(index) => {
                dbg!("Rule active", self.planning, self.rule_active, index, self.game.get_rule(index).get_positions());
                let rule = self.game.get_rule(self.rule_active);
                for pos in rule.get_positions() {
                    let index = pos.index();
                    let color = pos.get_sub_id(R, C)%2;
                    dbg!("Change color", index, color);
                    fields_guard.send(index, FieldMsg::ChangeColor(8+color));
                }
                
                self.rule_active = index;
            
                let rule = self.game.get_rule(self.rule_active);
                for pos in rule.get_positions() {
                    let index = pos.index();    
                    fields_guard.send(index, FieldMsg::ChangeColor(self.rule_active));
                }
            },

            AppMsg::Help => {
                popup("Help:\n\
                press 0-N to choose current setting value,\n\
                you can move with arrows to select field and press Enter to set value,\n\
                clicking on field will also set current setting value,\n\
                's' to save game,\n\
                'l' to load game from file,\n\
                'f' to finish game using solver,\n\
                'r' to toggle planning mode,\n\
                ");
            }

            AppMsg::Generate => {
                self.game = crate::logic::solver::Solver::generate(self.game.clone());
                for i in 0..N {
                    for j in 0..N {
                        let index = i * N + j;
                        let pos = crate::logic::board::Position::new(1 + j, 1 + i);
                        let sval = self.game.get_value(pos);
                        self.game.set_value(pos, sval);
                        fields_guard.send(index, FieldMsg::SetValue(sval));
                    }
                }
            },
            
            AppMsg::ToggleRules => {
                self.show_rules = !self.show_rules;
            }
        }
    }
}