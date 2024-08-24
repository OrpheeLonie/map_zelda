pub mod parse_image;

fn main() { let images = [ "map_zelda_images/20240310002429_1.jpg",
    "map_zelda_images/20240310002753_1.jpg", "map_zelda_images/20240310005040_1.jpg",
    "map_zelda_images/20240323155901_1.jpg", "map_zelda_images/20240323160144_1.jpg",
    "map_zelda_images/20240323160147_1.jpg", "map_zelda_images/20240323160151_1.jpg",
    "map_zelda_images/20240323160154_1.jpg", "map_zelda_images/20240323160157_1.jpg",
    "map_zelda_images/20240323160200_1.jpg", "map_zelda_images/20240323160211_1.jpg",
    "map_zelda_images/20240323160230_1.jpg", "map_zelda_images/20240323160232_1.jpg",
    "map_zelda_images/20240323160255_1.jpg", "map_zelda_images/20240323160318_1.jpg",
    "map_zelda_images/20240323160331_1.jpg", "map_zelda_images/20240323160350_1.jpg",
    "map_zelda_images/20240323160431_1.jpg", "map_zelda_images/20240323160452_1.jpg",
    "map_zelda_images/20240323160706_1.jpg", "map_zelda_images/20240323160735_1.jpg",
    "map_zelda_images/20240323160737_1.jpg", "map_zelda_images/20240323160752_1.jpg",
    "map_zelda_images/20240323160804_1.jpg", "map_zelda_images/20240323160835_1.jpg",
    "map_zelda_images/20240323160903_1.jpg", "map_zelda_images/20240323160907_1.jpg",
    "map_zelda_images/20240323160910_1.jpg" ];

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
        println!("\nimage name: {}", img_name);
        let img = parse_image::read_image(img_name).unwrap();
        match parse_image::add_image_to_map(&mut map, &img) {
            Ok(_) => println!("Ok"),
            Err(error) => println!("Error: {:?}", error),
        }
    }

    match map.save("map.png") {
        Ok(_) => println!("Save ok"),
        Err(error) => println!("Error: {:?}", error),
    };
}
