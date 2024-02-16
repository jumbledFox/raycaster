use image;

// Stores a texture as a vector of RGB values in column order for quick access!!!

pub struct Texture {
    pub data: Vec<u8>,
    pub width : usize,
    pub height: usize,
}

// This file has the number '3' in it an awful lot, that's because textures are stored as RGB, (3 elements per pixel).
impl Texture {
    pub fn from_file(path: &str) -> Texture {
        let image = image::open(path).unwrap().to_rgb8();
        let width : usize = image.width() .try_into().unwrap();
        let height: usize = image.height().try_into().unwrap();
        let image_data = image.into_raw();

        // Rotates the image so it's stored as a 1 dimensional array of columns, rather than rows. This lets us access it way faster!
        let mut rotated_image = vec![0; image_data.len()];

        for (i, pixel) in image_data.chunks_exact(3).enumerate() {
            let column = i % width;
            let row    = i / width;

            let index = (column * height + row)*3;
            rotated_image[index..index+3].clone_from_slice(pixel);
        }

        let along = 0.88;
        let column: usize = (along * (width-1) as f64).round() as usize; // TODO: Don't use 'as' here

        let slice_begin = ( column    * height) * 3;
        let slice_end   = ((column+1) * height) * 3;
        let s = &rotated_image[slice_begin..slice_end];

        println!("column: {:?}\nslice_begin: {:?}\nslice_end: {:?}\nslice: {:?}", column, slice_begin, slice_end, s);

        Texture { data: rotated_image, width, height }
    }

    // TODO:
    // pub fn get_slice(usize: column) ->  {

    // }
}