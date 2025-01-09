use crate::choose_color;
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, Position, positions::GridPosition};
use relm4::RelmWidgetExt;

const N: usize = 9;
const COLOR_LIST: [&str; 9] = [
    "red", "green", "purple", "orange", "pink", "brown", "yellow", "white", "grey",
];

#[derive(Debug)]
pub struct Field {
    pub value: usize,
    pub display_value: String,
    pub color: usize,
    pub index: usize,
}

#[derive(Debug)]
pub enum FieldMsg {
    ChangeColor(usize),
    ChangeValue,
    SetHints(Vec<usize>),
    SetValue(usize),
}

#[derive(Debug)]
pub enum FieldOutput {
    FieldClicked(usize),
}

pub struct FieldWidgets {
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

        Self {
            value: 0,
            display_value: String::from("_"),
            color,
            index,
        }
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
                    set_size_request: (100, 50),
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
                sender
                    .output(FieldOutput::FieldClicked(self.index))
                    .unwrap();
            }
            FieldMsg::SetValue(value) => {
                if value != N + 1 {
                    self.value = value;
                }
                self.display_value = match self.value {
                    0 => String::from("_"),
                    _ => self.value.to_string(),
                };
            }
            FieldMsg::ChangeColor(color) => {
                self.color = color;
            }
            FieldMsg::SetHints(hint) => {
                if self.value == 0 {
                    self.display_value = hint
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                }
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: FactorySender<Self>) {
        widgets.label.set_label(&self.display_value);
        widgets.label.set_css_classes(choose_color!(self.color));
    }
}
