use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::factory::positions::GridPosition;
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, Position};
use relm4::RelmWidgetExt;

const COLOR_LIST: [&str; 8] = [
    "red", "green", "purple", "orange", "pink", "brown", "black", "yellow",
];

macro_rules! choose_color {
    ($color_index:expr) => {
        &[&COLOR_LIST[$color_index]]
    };
}

#[derive(Debug)]
pub struct RuleButton {
    pub display_value: String,
    pub color: usize,
    pub index: usize,
}

#[derive(Debug)]
pub enum RuleMsg {
    Clicked,
}

#[derive(Debug)]
pub enum RuleOutput {
    RuleClicked(usize),
}

pub struct RuleWidgets {
    label: gtk::Button,
}

impl Position<GridPosition, DynamicIndex> for RuleButton {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index();
        let y = index;
        GridPosition {
            column: y as i32,
            row: 0_i32,
            width: 1,
            height: 1,
        }
    }
}

impl FactoryComponent for RuleButton {
    type Init = (String, usize);
    type Input = RuleMsg;
    type Output = RuleOutput;
    type CommandOutput = ();
    type Root = gtk::Box;
    type Widgets = RuleWidgets;
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

    fn init_model(
        (display_value, color): Self::Init,
        index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        let index = index.current_index();

        Self {
            display_value,
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
                    connect_clicked => RuleMsg::Clicked,
                    set_margin_all: 5,
                }
            }
        }

        RuleWidgets { label }
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        match msg {
            RuleMsg::Clicked => {
                sender.output(RuleOutput::RuleClicked(self.index)).unwrap();
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: FactorySender<Self>) {
        widgets.label.set_label(&self.display_value);
        widgets.label.set_css_classes(choose_color!(self.color));
    }
}
