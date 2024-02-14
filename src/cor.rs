use ggez::graphics;

fn converter_hsb_para_rgb(hue: f32, saturacao: f32, brilho: f32) -> (u8, u8, u8) {
    let croma = brilho * saturacao;
    let intensidade_cor = croma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs() - 1.0);
    let intensidade_luz = brilho - croma;

    let (r, g, b) = match hue {
        h if (0.0..60.0).contains(&h) => (croma, intensidade_cor, 0.0),
        h if (60.0..120.0).contains(&h) => (intensidade_cor, croma, 0.0),
        h if (120.0..180.0).contains(&h) => (0.0, croma, intensidade_cor),
        h if (180.0..240.0).contains(&h) => (0.0, intensidade_cor, croma),
        h if (240.0..300.0).contains(&h) => (intensidade_cor, 0.0, croma),
        h if (300.0..360.0).contains(&h) => (croma, 0.0, intensidade_cor),
        _ => (0.0, 0.0, 0.0),
    };

    let (r, g, b) = (
        ((r + intensidade_luz) * 255.0).round() as u8,
        ((g + intensidade_luz) * 255.0).round() as u8,
        ((b + intensidade_luz) * 255.0).round() as u8,
    );

    return (r, g, b);
}

pub fn gerar_cor(hue: f32) -> graphics::Color {
    let saturacao = 1.0;
    let brilho = 1.0;

    let (r, g, b) = converter_hsb_para_rgb(hue, saturacao, brilho);
    return graphics::Color::from_rgb(r, g, b);
}
