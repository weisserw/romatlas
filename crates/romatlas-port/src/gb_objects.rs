//! Selected DMG8x8 OAM composition for native ports, without PPU timing.
use crate::graphics::{Color, Frame};
use anyhow::{Result, ensure};
pub fn draw(
    frame: &mut Frame,
    tiles: &[[u8; 64]],
    palettes: &[[Color; 4]; 2],
    records: &[[u8; 4]],
) -> Result<()> {
    ensure!(records.len() <= 40, "DMG OAM has at most40 records");
    ensure!(
        records
            .iter()
            .all(|record| usize::from(record[2]) < tiles.len()),
        "object tile missing"
    );
    ensure!(
        tiles.iter().flatten().all(|&index| index < 4),
        "invalid2bpp object pixel"
    );
    for y in 0..frame.height as i32 {
        // Selection uses only Y and OAM order, even when X hides the object.
        let mut selected: Vec<_> = records
            .iter()
            .enumerate()
            .filter(|(_, record)| {
                let top = i32::from(record[0]) - 16;
                (top..top + 8).contains(&y)
            })
            .take(10)
            .collect();
        selected.sort_by_key(|(index, record)| (record[1], *index));
        for x in 0..frame.width as i32 {
            for &(_, record) in &selected {
                let dx = x - (i32::from(record[1]) - 8);
                if !(0..8).contains(&dx) {
                    continue;
                }
                let dy = y - (i32::from(record[0]) - 16);
                let sx = if record[3] & 32 != 0 { 7 - dx } else { dx };
                let sy = if record[3] & 64 != 0 { 7 - dy } else { dy };
                let pixel = tiles[record[2] as usize][(sy * 8 + sx) as usize];
                if pixel == 0 {
                    continue;
                }
                // Resolve OBJ winner before BG priority: a hidden winner masks
                // all lower-priority object pixels at this location.
                if record[3] & 128 == 0 || frame.background_index(x, y) == Some(0) {
                    frame.pixel(
                        x,
                        y,
                        palettes[usize::from((record[3] >> 4) & 1)][pixel as usize],
                    );
                }
                break;
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn palettes() -> [[Color; 4]; 2] {
        [[
            [255, 255, 255, 255],
            [1, 0, 0, 255],
            [2, 0, 0, 255],
            [3, 0, 0, 255],
        ]; 2]
    }
    #[test]
    fn smaller_x_wins_then_oam_tie_and_hidden_winner_masks_later_objects() {
        let mut frame = Frame::new(16, 8).unwrap();
        let tiles = [[1; 64], [2; 64]];
        draw(
            &mut frame,
            &tiles,
            &palettes(),
            &[[16, 12, 0, 0], [16, 10, 1, 0]],
        )
        .unwrap();
        assert_eq!(frame.pixels[4 * 4], 2);
        frame.clear([255; 4]);
        draw(
            &mut frame,
            &tiles,
            &palettes(),
            &[[16, 10, 0, 0], [16, 10, 1, 0]],
        )
        .unwrap();
        assert_eq!(frame.pixels[4 * 4], 1);
        frame.background_pixel(4, 0, 1, [255; 4]);
        draw(
            &mut frame,
            &tiles,
            &palettes(),
            &[[16, 10, 0, 128], [16, 10, 1, 0]],
        )
        .unwrap();
        assert_eq!(frame.pixels[4 * 4], 255);
    }
    #[test]
    fn offscreen_x_objects_consume_ten_object_selection_slots() {
        let mut frame = Frame::new(8, 8).unwrap();
        frame.clear([255; 4]);
        let mut records = vec![[16, 0, 0, 0]; 10];
        records.push([16, 8, 0, 0]);
        draw(&mut frame, &[[1; 64]], &palettes(), &records).unwrap();
        assert!(frame.pixels.iter().all(|&v| v == 255));
        records[0][0] = 0;
        draw(&mut frame, &[[1; 64]], &palettes(), &records).unwrap();
        assert_eq!(frame.pixels[0], 1);
    }
}
