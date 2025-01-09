#[macro_export]
macro_rules! for_pos {
    ($N:expr, $body:expr) => {
        for col in 1..=$N {
            for row in 1..=$N {
                let pos = Position::new(row, col).unwrap();
                $body(pos);
            }
        }
    };
}

#[macro_export]
macro_rules! choose_color {
    ($color_index:expr) => {
        &[&COLOR_LIST[$color_index]]
    };
}
