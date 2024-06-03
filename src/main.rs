use image::GenericImageView;
use image::Pixel;

// $@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'.

const CHARS_LEN: usize = 68;
const CHARS: [u8; CHARS_LEN] = [
   b'$', b'@', b'B', b'%', b'8', b'&', b'W', b'M', b'#', b'*', b'o', b'a', b'h', b'k', b'b', b'd', b'p', b'q', b'w', b'm', b'Z', b'O', b'0', b'Q', b'L', b'C', b'J', b'U', b'Y', b'X', b'z', b'c', b'v', b'u', b'n', b'x', b'r', b'j', b'f', b't', b'/', b'\\', b'|', b'(', b')', b'1', b'{', b'}', b'[', b']', b'?', b'-', b'_', b'+', b'~', b'<', b'>', b'i', b'!', b'l', b'I', b';', b':', b',', b'"', b'^', b'`', b'.'
];

const MAX_CANVAS_WIDTH: u32 = 40;
const MAX_CANVAS_HEIGHT: u32 = 20;

fn get_ascii_with_grayscale_value(grayscale_value: u8) -> char {
   let ascii_index = ((CHARS_LEN as f64) / 255.0) * grayscale_value as f64;
   CHARS[CHARS_LEN - 1 - ascii_index as usize] as char
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   // --image <image_path>
   let args: Vec<String> = std::env::args().collect();

   println!("{:?}", args);

   if args.len() != 3 {
      eprintln!("Usage: --image <image_path>");
      return Err("Invalid arguments 1".into());
   }

   if args[1] != "--image" {
      eprintln!("Usage: --image <image_path>");
      return Err("Invalid arguments 2".into());
   }

   let image = &args[2];

   let target_image = image::open(image)?.grayscale();

   let (width, height) = target_image.dimensions();
   let aspect_ratio = width as f64 / height as f64;

   let new_width = width / MAX_CANVAS_WIDTH * MAX_CANVAS_WIDTH;
   let new_height = (new_width as f64 / aspect_ratio) as u32;

   let target_image = target_image.resize_exact(new_width, new_height as u32, image::imageops::FilterType::Nearest);

   let width_batch_size = new_width / MAX_CANVAS_WIDTH;
   let height_batch_size = new_height / MAX_CANVAS_HEIGHT;

   for y in 0..MAX_CANVAS_HEIGHT {
      for x in 0..MAX_CANVAS_WIDTH {
         let mut grayscale_value: u32 = 0;

         for j in 0..height_batch_size {
            for i in 0..width_batch_size {
               let pixel_x = x * width_batch_size + i;
               let pixel_y = y * height_batch_size + j;

               if pixel_x >= new_width || pixel_y >= new_height {
                  break;
               }

               let pixel = target_image.get_pixel(pixel_x, pixel_y);
               grayscale_value += pixel.to_luma().0[0] as u32;
            }
         }

         grayscale_value = grayscale_value / (width_batch_size * height_batch_size);

         print!("{}", get_ascii_with_grayscale_value(grayscale_value as u8));
      }

      println!();
   }

   Ok(())
}
