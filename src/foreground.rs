use image::DynamicImage;
struct Foreground {
    image: DynamicImage,
    style: Option<Style2Apply>,
}

struct Style2Apply {
    brighten: Option<i32>,
}
