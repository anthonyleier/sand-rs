use ggez::graphics;

pub fn gerar_cor(hue: u8) -> graphics::Color {
    let (r, g, b) = match hue {
        h if (0..25).contains(&h) => (250, 250, 110),
        h if (25..50).contains(&h) => (196, 236, 116),
        h if (50..75).contains(&h) => (146, 220, 126),
        h if (75..100).contains(&h) => (100, 201, 135),
        h if (100..125).contains(&h) => (57, 180, 142),
        h if (125..150).contains(&h) => (8, 159, 143),
        h if (150..175).contains(&h) => (0, 137, 138),
        h if (175..200).contains(&h) => (8, 115, 127),
        h if (200..225).contains(&h) => (33, 93, 110),
        h if (225..=255).contains(&h) => (42, 72, 88),
        _ => (255, 0, 0),
    };
    return graphics::Color::from_rgb(r, g, b);
}
