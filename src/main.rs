pub mod parse_image;

fn main() {
    let images = ["images/20240310002429_1.jpg", "images/20240323155901_1.jpg"];

    let first_img = parse_image::read_image(images[0]).unwrap();
    let map_top_left_corner = parse_image::find_map_top_left_corner(&first_img);
    let map_size = parse_image::find_map_size(&first_img, map_top_left_corner);
    let cursor_top_left_corner = parse_image::find_cursor_location(&first_img, map_top_left_corner, map_size);
    let cursor_top_left_corner_absolute = (cursor_top_left_corner.0 + map_top_left_corner.0, cursor_top_left_corner.1 + map_top_left_corner.1);
    let cursor_size = parse_image::find_cursor_size(&first_img, cursor_top_left_corner_absolute);
    let map_dim = parse_image::get_map_dimmensions(map_size, cursor_size);

    println!("{}, {}", map_dim.0 * first_img.width(), map_dim.1 * first_img.height());
    let mut map = image::RgbImage::new(map_dim.0 * first_img.width(), map_dim.1 * first_img.height());

    for img_name in images {
        let img = parse_image::read_image(img_name).unwrap();
        match parse_image::add_image_to_map(&mut map, &img) {
            Ok(_) => println!("Ok"),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
