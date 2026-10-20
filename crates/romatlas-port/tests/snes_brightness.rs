use romatlas_port::snes::{
    color, color_with_brightness,
    ppu::{Composition, Pixel},
};

#[test]
fn brightness_keeps_five_bit_precision_through_output_conversion() {
    let packed = 2 | (3 << 5) | (31 << 10);
    assert_eq!(color(packed), [16, 25, 255, 255]);
    assert_eq!(color_with_brightness(packed, 0), [0, 0, 0, 255]);
    assert_eq!(color_with_brightness(packed, 1), [1, 2, 17, 255]);
    assert_eq!(color_with_brightness(packed, 10), [11, 16, 170, 255]);
    assert_eq!(color_with_brightness(packed, 255), color(packed));
    let main = Pixel {
        color: packed,
        rank: 1,
        layer: 0,
        math: false,
    };
    assert_eq!(
        Composition {
            brightness: 10,
            ..Default::default()
        }
        .compose(main, main),
        [11, 16, 170, 255]
    );
    assert_eq!(
        Composition {
            brightness: 10,
            force_blank: true,
            ..Default::default()
        }
        .compose(main, main),
        [0, 0, 0, 255]
    );
}
