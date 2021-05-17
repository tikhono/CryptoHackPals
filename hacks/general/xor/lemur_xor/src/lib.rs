#[cfg(test)]
mod tests {

    #[test]
    fn capture_the_flag() {
        use image::GenericImageView;

        let flag = image::open("../../../../hacks/general/xor/lemur_xor/flag.png").unwrap();

        let lemur = image::open("../../../../hacks/general/xor/lemur_xor/lemur.png").unwrap();

        let mut imgbuf = image::ImageBuffer::new(flag.width(), flag.height());

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = flag.get_pixel(x, y)[0] ^ lemur.get_pixel(x, y)[0];
            let g = flag.get_pixel(x, y)[1] ^ lemur.get_pixel(x, y)[1];
            let b = flag.get_pixel(x, y)[2] ^ lemur.get_pixel(x, y)[2];
            *pixel = image::Rgb([r, g, b]);
        }

        imgbuf.save("out.png").unwrap();
    }
}
