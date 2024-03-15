use image::io::Reader as ImageReader;

pub fn read_image(file_name: &str) -> Result<image::DynamicImage, image::ImageError> {
    let img = ImageReader::open(file_name)?.decode()?;

    Ok(img)
}

pub fn find_location_in_map(_img: image::DynamicImage) -> (u32, u32) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_image() {
        // Given
        // When
        let img = read_image("images/20240308232334_1.jpg").unwrap();

        // Then
        assert_eq!(img.width(), 1152);
        assert_eq!(img.height(), 1080);
    }

    #[test]
    fn test_find_location_in_map() {
    }
}
