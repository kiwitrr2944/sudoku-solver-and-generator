mod gui;
mod logic;
use gui::app::App;
use relm4::RelmApp;

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
