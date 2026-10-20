//! Original synthetic tests; no console-core implementation or game assets.
use romatlas_port::snes::ppu::{Object, object_scanline, object_scanline_limited};

fn object() -> Object {
    Object {
        x: 0,
        y: 0,
        tile: 2,
        name_table: false,
        palette: 0,
        priority: 0,
        flip_x: false,
        flip_y: false,
        large: false,
    }
}

#[test]
fn range_limit_counts_transparent_and_minus_256_but_not_other_hidden_objects() {
    let mut vram = [0; 65536];
    vram[0] = 255;
    let cgram = [31; 256];
    let mut objects = vec![object(); 32];
    objects.push(Object {
        tile: 0,
        ..object()
    });
    let line = object_scanline_limited(&objects, &vram, &cgram, 0, 0, [2, 5, 8, 11]).unwrap();
    assert!(line.range_over);
    assert!(!line.time_over);
    assert!(line.pixels.iter().all(Option::is_none));
    // These are hardware-eligible despite being offscreen.
    for obj in &mut objects[..32] {
        obj.x = -256;
    }
    assert!(
        object_scanline_limited(&objects, &vram, &cgram, 0, 0, [2, 5, 8, 11])
            .unwrap()
            .range_over
    );
    for obj in &mut objects[..32] {
        obj.x = -8;
    }
    let line = object_scanline_limited(&objects, &vram, &cgram, 0, 0, [2, 5, 8, 11]).unwrap();
    assert!(!line.range_over);
    assert_eq!(line.pixels[0].unwrap().color, 31);
    objects[0].x = -7;
    objects[0].tile = 0;
    assert!(
        object_scanline_limited(&objects[..1], &vram, &cgram, 0, 0, [2, 5, 8, 11])
            .unwrap()
            .pixels[0]
            .is_some()
    );
}

#[test]
fn reverse_fetch_discards_right_column_even_when_flipped() {
    let mut vram = [0; 65536];
    vram[0] = 255; // left source tile: index 1
    vram[33] = 255; // right source tile: index 2
    let mut cgram = [0; 256];
    cgram[193] = 31;
    cgram[194] = 992;
    // The last 17 transparent objects consume 33 slivers. Only the screen-left
    // column of the first object fits, even though it samples the flipped right tile.
    let mut objects = vec![
        Object {
            x: 64,
            large: true,
            ..object()
        };
        18
    ];
    objects[0] = Object {
        tile: 0,
        large: true,
        flip_x: true,
        palette: 4,
        priority: 3,
        ..object()
    };
    objects[17].large = false;
    let line = object_scanline_limited(&objects, &vram, &cgram, 0, 0, [2, 5, 8, 11]).unwrap();
    assert!(!line.range_over);
    assert!(line.time_over);
    for pixel in &line.pixels[..8] {
        let pixel = pixel.unwrap();
        assert_eq!((pixel.color, pixel.rank, pixel.math), (992, 11, true));
    }
    assert!(line.pixels[8..].iter().all(Option::is_none));
    // Exactly 34 fetches is legal; transparent columns still count.
    objects.remove(17);
    let line = object_scanline_limited(&objects, &vram, &cgram, 0, 0, [2, 5, 8, 11]).unwrap();
    assert!(!line.time_over);
    assert_eq!(line.pixels[8].unwrap().color, 31);
}

#[test]
fn hidden_columns_consume_fetches_only_at_minus_256() {
    let vram = [0; 65536];
    let cgram = [0; 256];
    let mut objects = vec![
        Object {
            x: -256,
            large: true,
            ..object()
        };
        18
    ];
    assert!(
        object_scanline_limited(&objects, &vram, &cgram, 0, 0, [2, 5, 8, 11])
            .unwrap()
            .time_over
    );
    for object in &mut objects {
        object.x = -8;
    }
    assert!(
        !object_scanline_limited(&objects, &vram, &cgram, 0, 0, [2, 5, 8, 11])
            .unwrap()
            .time_over
    );
}

#[test]
fn below_limits_matches_unbounded_sampling_with_wrapped_y_and_rotation_order() {
    let mut vram = [0; 65536];
    for row in 0..8 {
        vram[row * 2] = 255;
    }
    let cgram = std::array::from_fn(|i| i as u16);
    let objects = [
        Object {
            x: -3,
            y: 250,
            tile: 0,
            large: true,
            ..object()
        },
        Object {
            tile: 0,
            palette: 4,
            priority: 3,
            ..object()
        },
    ];
    for ordered in [objects, [objects[1], objects[0]]] {
        let bounded =
            object_scanline_limited(&ordered, &vram, &cgram, 0, 0, [2, 5, 8, 11]).unwrap();
        let unbounded = object_scanline(&ordered, &vram, &cgram, 0, 0, [2, 5, 8, 11]).unwrap();
        assert!(!bounded.range_over && !bounded.time_over);
        for (left, right) in bounded.pixels.iter().zip(unbounded) {
            assert_eq!(
                left.map(|p| (p.color, p.rank, p.layer, p.math)),
                right.map(|p| (p.color, p.rank, p.layer, p.math))
            );
        }
    }
    assert!(object_scanline_limited(&[object(); 129], &vram, &cgram, 0, 0, [2, 5, 8, 11]).is_err());
    assert!(
        object_scanline_limited(
            &[Object { x: 256, ..object() }],
            &vram,
            &cgram,
            0,
            0,
            [2, 5, 8, 11]
        )
        .is_err()
    );
}
