//pub mod assets {
// Load player images
#[macro_export]
macro_rules! image_list {
    () => {{
        let img1 = include_bytes!("img/player-001.png");
        let img2 = include_bytes!("img/player-002.png");
        vec![img1.as_slice(), img2.as_slice()]
    }};
}

#[macro_export]
macro_rules! font_list {
    () => {{
        let dejavusans = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/fonts/DejaVuSans.ttf"
        ));
        let dejavusans_bold = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/fonts/DejaVuSans-Bold.ttf"
        ));
        let hydrogen_whiskey = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/fonts/HydrogenWhiskey.otf"
        ));
        let stormfaze = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/fonts/Stormfaze.otf"
        ));
        vec![
            dejavusans.as_slice(),
            dejavusans_bold.as_slice(),
            hydrogen_whiskey.as_slice(),
            stormfaze.as_slice(),
        ]
    }};
}
//}
