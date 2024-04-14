// https://www.utf8-chartable.de/unicode-utf8-table.pl?number=1024
// https://medium.com/@Andrew_Mc/the-%E1%97%AA-%C8%B6%D1%87-of-unicode-homoglyphs-41ee73b57913
// https://github.com/wisespace-io/nettfiske
fn main() {
    let glyph1 = 'X';
    let glyph2 = 'Ｘ';

    let mut byte_buffer1 = [0; 4];
    let mut byte_buffer2 = [0; 4];

    println!();

    if glyph1 == glyph2 {
        println!(
            "{} (bytes:{:#?}) is equal to {} (bytes:{:#?})",
            glyph1,
            glyph1.encode_utf8(&mut byte_buffer1).as_bytes(),
            glyph2,
            glyph2.to_string().as_bytes(),
        );
    } else {
        println!(
            "{} (bytes:{:#?}) is not equal to {} (bytes:{:#?})",
            glyph1,
            glyph1.to_string().as_bytes(),
            glyph2,
            glyph2.encode_utf8(&mut byte_buffer2).as_bytes(),
        );
    }
}
