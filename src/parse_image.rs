use image::io::Reader as ImageReader;

/// Compare 2 pixels while ignoring a diff of less than `accuracy`
fn pixel_are_equals(px1: &image::Rgb<u8>, px2: &image::Rgb<u8>, accuracy: i32) -> bool {
    let chan1 = px1.0;
    let chan2 = px2.0;

    for i in 0..3 {
        let diff: i32 = chan1[i] as i32 - chan2[i] as i32;
        if -accuracy > diff || accuracy < diff {
            return false
        }
    }

    true
}


/// Try to open the image at `file_name`
pub fn read_image(file_name: &str) -> Result<image::DynamicImage, image::ImageError> {
    let img = ImageReader::open(file_name)?.decode()?;

    Ok(img)
}

/// Search for the top left corner of the map (first not black pixel) and return
/// its coordinates
pub fn find_map_top_left_corner(img: &image::DynamicImage) -> (u32, u32) {
    let rgb_image = img.as_rgb8().unwrap();
    let mut x = 0;
    let mut y = 0;
    let black_pixel = image::Rgb([0, 0, 0]);

    while pixel_are_equals(rgb_image.get_pixel(x, y), &black_pixel, 10) {
        x += 1;
        if y < x {
            y += 1;
            x = 0;
        }
    }
    (x, y)
}

/// find the size of the map
pub fn find_map_size(img: &image::DynamicImage, coordinates: (u32, u32)) -> (u32, u32) {
    let mut width = 0;
    let mut height = 0;
    let rgb_image = img.as_rgb8().unwrap();
    let black_pixel = image::Rgb([0, 0, 0]);

    while !pixel_are_equals(rgb_image.get_pixel(coordinates.0 + width, coordinates.1), &black_pixel, 10) {
        width += 1;
    }

    while !pixel_are_equals(rgb_image.get_pixel(coordinates.0, coordinates.1 + height), &black_pixel, 10) {
        height += 1;
    }

    (width - 1, height - 1)
}

/// Find the location of the cursor
/// return the position divided by the size to get the index of the screen
pub fn find_cursor_location(img: &image::DynamicImage, map_top_left_corner: (u32, u32), map_size: (u32, u32)) -> (u32, u32) {
    let (map_corner_x, map_corner_y) = map_top_left_corner;
    let rgb_image = img.as_rgb8().unwrap();
    let top_left_pixel = rgb_image.get_pixel(map_corner_x, map_corner_y);

    for x in 0..map_size.0 {
        for y in 0..map_size.1 {
            if !pixel_are_equals(rgb_image.get_pixel(map_corner_x + x, map_corner_y + y), top_left_pixel, 20) {
                return (x, y);
            }
        }
    }
    return (0, 0)
}

pub fn find_cursor_size(img: &image::DynamicImage, cursor_top_left_corner: (u32, u32)) -> (u32, u32) {
    let rgb_image = img.as_rgb8().unwrap();
    let cursor_color = rgb_image.get_pixel(cursor_top_left_corner.0, cursor_top_left_corner.1);

    let mut width = 0;
    let mut height = 0;

    while pixel_are_equals(rgb_image.get_pixel(cursor_top_left_corner.0 + width, cursor_top_left_corner.1), cursor_color, 20) {
        width += 1;
    }
    while pixel_are_equals(rgb_image.get_pixel(cursor_top_left_corner.0, cursor_top_left_corner.1 + height), cursor_color, 20) {
        height += 1;
    }

    (width, height)
}

pub fn get_cursor_location_on_map(cursor_top_left_corner: (u32, u32), size: (u32, u32)) -> (u32, u32) {
    let (width, height) = size;

    ((cursor_top_left_corner.0 + width/2) / width, (cursor_top_left_corner.1 + height/2) / height)
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
        let (x, y) = find_map_top_left_corner(&img);

        // Then
        assert_eq!(72, x);
        assert_eq!(108, y);
    }

    #[test]
    fn test_find_map_size() {
        // Given
        let img = read_image("images/20240310002429_1.jpg").unwrap();
        let coord = find_map_top_left_corner(&img);

        // When
        let size = find_map_size(&img, coord);

        // Then
        assert_eq!((287, 143), size);
    }

    #[test]
    fn test_find_cursor_location() {
        // Given
        let img = read_image("images/20240310002429_1.jpg").unwrap();
        let coord = find_map_top_left_corner(&img);
        let size = find_map_size(&img, coord);

        // When
        let pos = find_cursor_location(&img, coord, size);

        // Then
        assert_eq!((23, 36), pos);
    }

    #[test]
    fn test_find_cursor_size() {
        // Given
        let img = read_image("images/20240310002429_1.jpg").unwrap();
        let map_top_left_corner = find_map_top_left_corner(&img);
        let map_size = find_map_size(&img, map_top_left_corner);
        let cursor_top_left_corner = find_cursor_location(&img, map_top_left_corner, map_size);

        let cursor_top_left_corner_absolute = (cursor_top_left_corner.0 + map_top_left_corner.0, cursor_top_left_corner.1 + map_top_left_corner.1);

        // When
        let cursor_size = find_cursor_size(&img, cursor_top_left_corner_absolute);

        // Then
        assert_eq!((13, 14), cursor_size);
    }

    #[test]
    fn test_find_cursor_location_on_map() {
        // Given
        let img = read_image("images/20240310002429_1.jpg").unwrap();
        let map_top_left_corner = find_map_top_left_corner(&img);
        let map_size = find_map_size(&img, map_top_left_corner);
        let cursor_top_left_corner = find_cursor_location(&img, map_top_left_corner, map_size);

        let cursor_top_left_corner_absolute = (cursor_top_left_corner.0 + map_top_left_corner.0, cursor_top_left_corner.1 + map_top_left_corner.1);
        let cursor_size = find_cursor_size(&img, cursor_top_left_corner_absolute);
        // When
        let cursor_pos = get_cursor_location_on_map(cursor_top_left_corner, cursor_size);

        // Then
        assert_eq!((2, 3), cursor_pos);
    }
}
