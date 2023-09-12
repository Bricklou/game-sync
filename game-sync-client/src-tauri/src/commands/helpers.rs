use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[tauri::command]
pub async fn get_image_dominant(src: &str) -> Result<Color, String> {
    let response = reqwest::get(src)
        .await
        .map_err(|e| format!("failed to get image: {}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("failed to get image bytes: {}", e))?;

    let image =
        image::load_from_memory(&bytes).map_err(|e| format!("failed to load image: {}", e))?;

    let mut colors = dominant_color::get_colors(image.to_rgb8().into_raw().as_slice(), false);

    if colors.len() < 3 {
        return Err("failed to get dominant colors".to_string());
    }

    let color = Color {
        r: colors.pop().unwrap(),
        g: colors.pop().unwrap(),
        b: colors.pop().unwrap(),
    };

    println!("colors: {:?}", color);

    Ok(color)
}
