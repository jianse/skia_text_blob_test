use std::{fs::File, io::Read};

use eyre::{Ok, Result};
use skia_safe::{Font, FontMgr, Point, TextBlob, Typeface};

#[test]
fn test_text_blob() -> Result<()> {
    let text = "string".to_string();
    let tf = get_typeface()?;
    let font = Font::from_typeface(tf, 10.0);

    let glyph_count = font.count_text(&text);
    dbg!(glyph_count);

    // this line will fall on linux. but works fine on windows
    // and make parameter `text` to `&text` fix this
    // but why?
    let text_blob =
        TextBlob::from_pos_text(text, &[Point { x: 0.0, y: 0.0 }].repeat(glyph_count), &font)
            .unwrap();
    dbg!(text_blob);
    Ok(())
}

fn get_typeface() -> Result<Typeface> {
    let fm = FontMgr::new();
    let mut file = File::open("UbuntuMono-R.ttf")?;
    let mut bytes = Vec::new();
    let _ = file.read_to_end(&mut bytes)?;
    let tf = fm.new_from_data(&bytes, 0).unwrap();
    Ok(tf)
}
