use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::positions::GridPosition;
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, Position};
use relm4::RelmWidgetExt;

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
                sender.output(FieldOutput::FieldClicked(self.index)).unwrap();
            }
            FieldMsg::SetValue(value) => {
                self.value = value;
                dbg!(self.value, self.index, "set");
                self.display_value = match value {
                    0 => String::from("_"),
                    _ => value.to_string(),
                };
            },
            FieldMsg::ChangeColor(color) => {
                self.color = color;
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: FactorySender<Self>) {
        widgets.label.set_label(&self.display_value);
        widgets.label.set_css_classes(choose_color!(self.color));
    }
}