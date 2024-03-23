use image::io::Reader as ImageReader;

/// Try to open the image at `file_name`
pub fn read_image(file_name: &str) -> Result<image::DynamicImage, image::ImageError> {
    let img = ImageReader::open(file_name)?.decode()?;

    Ok(img)
}

/// Search for the top left corner of the map (first not black pixel) and return
/// its coordinates
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

pub fn find_map_size(img: &image::DynamicImage, coordinates: (u32, u32)) -> (u32, u32) {
    let mut width = 0;
    let mut height = 0;
    let rgb_image = img.as_rgb8().unwrap();

    while !rgb_image.get_pixel(coordinates.0 + width, coordinates.1).eq(&image::Rgb([0, 0, 0])) {
        width += 1;
    }

    while !rgb_image.get_pixel(coordinates.0, coordinates.1 + height).eq(&image::Rgb([0, 0, 0])) {
        height += 1;
    }

    return (width, height)
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

    #[test]
    fn test_find_map_size() {
        // Given
        let img = read_image("images/20240310002429_1.jpg").unwrap();
        let coord = find_location_in_map(&img);

        // When
        let size = find_map_size(&img, coord);

        // Then
        assert_eq!((288, 146), size);
    }
}
