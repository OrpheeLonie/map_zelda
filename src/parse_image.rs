use image::io::Reader as ImageReader;

pub fn read_image(file_name: &str) -> Result<image::DynamicImage, image::ImageError> {
    let img = ImageReader::open(file_name)?.decode()?;

    Ok(img)
}

pub fn find_location_in_map(img: &image::DynamicImage) -> (u32, u32) {
    let rgb_image = img.as_rgb8().unwrap();
    let mut x = 0;
    let mut y = 0;
    while rgb_image.get_pixel(x, y).eq(&image::Rgb([0, 0, 0])) {
        x += 1;
        if y < x {
            y += 1;
            x = 0;
        }
    }
    (x, y)
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
        // Given
        let img = read_image("images/20240310002429_1.jpg").unwrap();

        // When
        let (x, y) = find_location_in_map(&img);

        // Then
        assert_eq!(72, x);
        assert_eq!(107, y);
    }
}
