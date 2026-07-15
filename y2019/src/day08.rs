#![allow(clippy::naive_bytecount)]

use advent_of_code_macros::aoc_solver;

const WHITE_PIXEL: u8 = b'0';
const BLACK_PIXEL: u8 = b'1';
const TRANSPARENT_PIXEL: u8 = b'2';
const WHITE_PIXEL_CHAR: char = ' ';
const BLACK_PIXEL_CHAR: char = '#';
const TRANSPARENT_PIXEL_CHAR: char = ' ';

struct Image {
    width: usize,
    height: usize,
    layers: Vec<Vec<u8>>,
}

impl Image {
    fn new(width: usize, height: usize, data: &str) -> Self {
        let layer_size = width * height;
        let layers = data
            .as_bytes()
            .chunks(layer_size)
            .map(<[u8]>::to_vec)
            .collect();
        Self {
            width,
            height,
            layers,
        }
    }

    fn count_digit(&self, digit: u8) -> Vec<usize> {
        self.layers
            .iter()
            .map(|layer| layer.iter().filter(|&&d| d == digit).count())
            .collect()
    }

    fn compress_layers(&self) -> Self {
        let mut final_image = vec![b'2'; self.width * self.height]; // Start with all transparent
        for layer in &self.layers {
            for (i, &pixel) in layer.iter().enumerate() {
                if final_image[i] == TRANSPARENT_PIXEL && pixel != TRANSPARENT_PIXEL {
                    final_image[i] = pixel; // Set the pixel if it's not transparent
                }
            }
        }
        Self {
            width: self.width,
            height: self.height,
            layers: vec![final_image],
        }
    }

    fn display(&self) -> String {
        let mut output = String::new();
        output.push('\n'); // Add a newline at the beginning for better formatting
        for layer in &self.layers {
            for (i, &pixel) in layer.iter().enumerate() {
                if i > 0 && i % self.width == 0 {
                    output.push('\n');
                }
                output.push(match pixel {
                    WHITE_PIXEL => WHITE_PIXEL_CHAR,
                    BLACK_PIXEL => BLACK_PIXEL_CHAR,
                    TRANSPARENT_PIXEL => TRANSPARENT_PIXEL_CHAR,
                    _ => ' ',
                });
            }
            output.push('\n');
        }
        output
    }
}

#[aoc_solver(file = "inputs/2019/day08.txt", expected = 1920)]
fn part_1(input: &str) -> usize {
    let image = Image::new(25, 6, input.trim());
    let count_0 = image.count_digit(b'0');
    let min_layer_index = count_0
        .iter()
        .enumerate()
        .min_by_key(|&(_, &count)| count)
        .map(|(index, _)| index)
        .expect("No layers found");
    let layer = &image.layers[min_layer_index];
    let count_1 = layer.iter().filter(|&&d| d == BLACK_PIXEL).count();
    let count_2 = layer.iter().filter(|&&d| d == TRANSPARENT_PIXEL).count();
    count_1 * count_2
}

const PART_2_EXPECTED: &str = "
###   ##  #  # #     ##  
#  # #  # #  # #    #  # 
#  # #    #  # #    #  # 
###  #    #  # #    #### 
#    #  # #  # #    #  # 
#     ##   ##  #### #  # 
"; // PCULA

#[aoc_solver(file = "inputs/2019/day08.txt", expected = PART_2_EXPECTED)]
fn part_2(input: &str) -> String {
    let image = Image::new(25, 6, input.trim());
    let compressed_image = image.compress_layers();
    compressed_image.display()
}
