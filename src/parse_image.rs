use image::io::Reader as ImageReader;
use image::GenericImage;

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
pub fn read_image(file_name: &str) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, image::ImageError> {
    let dynamic_img = ImageReader::open(file_name)?.decode()?;
    let img = dynamic_img.as_rgb8().unwrap();


    Ok(img.clone())
}

/// Search for the top left corner of the map (first not black pixel) and return
/// its coordinates
pub fn find_map_top_left_corner(img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> (u32, u32) {
    let mut x = 0;
    let mut y = 0;
    let black_pixel = image::Rgb([0, 0, 0]);

    while pixel_are_equals(img.get_pixel(x, y), &black_pixel, 10) {
        x += 1;
        if y < x {
            y += 1;
            x = 0;
        }
    }
    (x, y)
}

/// find the size of the map
pub fn find_map_size(img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, coordinates: (u32, u32)) -> (u32, u32) {
    let mut width = 0;
    let mut height = 0;
    let black_pixel = image::Rgb([0, 0, 0]);

    while !pixel_are_equals(img.get_pixel(coordinates.0 + width, coordinates.1), &black_pixel, 10) {
        width += 1;
    }

    while !pixel_are_equals(img.get_pixel(coordinates.0, coordinates.1 + height), &black_pixel, 10) {
        height += 1;
    }

    (width - 1, height - 1)
}

/// Find the location of the cursor's top left corner
pub fn find_cursor_location(img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, map_top_left_corner: (u32, u32), map_size: (u32, u32)) -> (u32, u32) {
    let (map_corner_x, map_corner_y) = map_top_left_corner;
    let top_left_pixel = img.get_pixel(map_corner_x, map_corner_y);

    for x in 0..map_size.0 {
        for y in 0..map_size.1 {
            if !pixel_are_equals(img.get_pixel(map_corner_x + x, map_corner_y + y), top_left_pixel, 20) {
                return (x, y);
            }
        }
    }
    return (0, 0)
}

/// Get the dimmension of the cursor
pub fn find_cursor_size(img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, cursor_top_left_corner: (u32, u32)) -> (u32, u32) {
    let cursor_color = img.get_pixel(cursor_top_left_corner.0, cursor_top_left_corner.1);

    let mut width = 0;
    let mut height = 0;

    while pixel_are_equals(img.get_pixel(cursor_top_left_corner.0 + width, cursor_top_left_corner.1), cursor_color, 25) {
        width += 1;
    }
    while pixel_are_equals(img.get_pixel(cursor_top_left_corner.0, cursor_top_left_corner.1 + height), cursor_color, 25) {
        height += 1;
    }

    (width, height)
}

/// return the position divided by the size to get the index of the cursor on the map
pub fn get_cursor_location_on_map(cursor_top_left_corner: (u32, u32), size: (u32, u32)) -> (u32, u32) {
    let (width, height) = size;

    let width = width + 5;
    let height = height + 4;

    ((cursor_top_left_corner.0 + width/2) / width, (cursor_top_left_corner.1 + height/2) / height)
}

/// return the number of tile on the map
pub fn get_map_dimmensions(map_size: (u32, u32), cursor_size: (u32, u32)) -> (u32, u32) {
    (map_size.0 / (cursor_size.0 + 5), map_size.1 / (cursor_size.1 + 4))
}

pub fn add_image_to_map(map: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> image::ImageResult<()> {
    let map_top_left_corner = find_map_top_left_corner(&img);
    let map_size = find_map_size(&img, map_top_left_corner);
    let cursor_top_left_corner = find_cursor_location(&img, map_top_left_corner, map_size);
    let cursor_top_left_corner_absolute = (cursor_top_left_corner.0 + map_top_left_corner.0, cursor_top_left_corner.1 + map_top_left_corner.1);
    let cursor_size = find_cursor_size(&img, cursor_top_left_corner_absolute);
    let cursor_pos = get_cursor_location_on_map(cursor_top_left_corner, cursor_size);

    map.copy_from(img, cursor_pos.0 * map_size.0, cursor_size.1 * map_size.1)?;

    Ok(())
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
        assert_eq!((1, 2), cursor_pos);
    }

    #[test]
    fn test_find_cursor_location_on_map_bottom_right_tile() {
        // Given
        let img = read_image("images/20240323155901_1.jpg").unwrap();

        // When
        let map_top_left_corner = find_map_top_left_corner(&img);
        let map_size = find_map_size(&img, map_top_left_corner);
        let cursor_top_left_corner = find_cursor_location(&img, map_top_left_corner, map_size);

        let cursor_top_left_corner_absolute = (cursor_top_left_corner.0 + map_top_left_corner.0, cursor_top_left_corner.1 + map_top_left_corner.1);
        let cursor_size = find_cursor_size(&img, cursor_top_left_corner_absolute);
        let cursor_pos = get_cursor_location_on_map(cursor_top_left_corner, cursor_size);

        // Then
        assert_eq!((72, 108), map_top_left_corner);
        assert_eq!((287, 143), map_size);
        assert_eq!((275, 126), cursor_top_left_corner);
        assert_eq!((347, 234), cursor_top_left_corner_absolute);
        assert_eq!((13, 14), cursor_size);
        assert_eq!((15, 7), cursor_pos);
    }

    #[test]
    fn test_find_map_dimension() {
         // Given
        let img = read_image("images/20240310002429_1.jpg").unwrap();
        let map_top_left_corner = find_map_top_left_corner(&img);
        let map_size = find_map_size(&img, map_top_left_corner);
        let cursor_top_left_corner = find_cursor_location(&img, map_top_left_corner, map_size);

        let cursor_top_left_corner_absolute = (cursor_top_left_corner.0 + map_top_left_corner.0, cursor_top_left_corner.1 + map_top_left_corner.1);
        let cursor_size = find_cursor_size(&img, cursor_top_left_corner_absolute);

        // When
        let map_dim = get_map_dimmensions(map_size, cursor_size);

        // Then
        assert_eq!((15, 7), map_dim);
   }

    #[test]
    fn test_find_map_dimension_2() {
         // Given
        let img = read_image("images/20240323155901_1.jpg").unwrap();
        let map_top_left_corner = find_map_top_left_corner(&img);
        let map_size = find_map_size(&img, map_top_left_corner);
        let cursor_top_left_corner = find_cursor_location(&img, map_top_left_corner, map_size);

        let cursor_top_left_corner_absolute = (cursor_top_left_corner.0 + map_top_left_corner.0, cursor_top_left_corner.1 + map_top_left_corner.1);
        let cursor_size = find_cursor_size(&img, cursor_top_left_corner_absolute);

        // When
        let map_dim = get_map_dimmensions(map_size, cursor_size);

        // Then
        assert_eq!((15, 7), map_dim);
   }
}
