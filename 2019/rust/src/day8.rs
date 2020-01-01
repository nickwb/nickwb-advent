use crate::util;

type ImageDimension = usize;

struct SpaceImage<'a> {
    digits: &'a str,
    width: ImageDimension,
    height: ImageDimension,
}

struct LayerIter<'a> {
    image: &'a SpaceImage<'a>,
    index: Option<ImageDimension>,
}

struct Layer<'a> {
    width: ImageDimension,
    height: ImageDimension,
    pixels: &'a str,
}

enum PixelType {
    Black,
    White,
    Transparent,
}

impl<'a> SpaceImage<'a> {
    fn new(digits: &'a str, width: ImageDimension, height: ImageDimension) -> SpaceImage<'a> {
        SpaceImage {
            digits,
            width,
            height,
        }
    }

    fn layer_size(&self) -> ImageDimension {
        self.width * self.height
    }

    fn layer_count(&self) -> ImageDimension {
        self.digits.len() / self.layer_size()
    }

    fn layers(&'a self) -> LayerIter<'a> {
        LayerIter {
            image: self,
            index: None,
        }
    }

    fn get_layer(&'a self, index: ImageDimension) -> Option<Layer<'a>> {
        if index >= self.layer_count() {
            return None;
        }

        let begin = index * self.layer_size();
        let end = begin + self.layer_size();

        Some(Layer {
            width: self.width,
            height: self.height,
            pixels: &self.digits[begin..end],
        })
    }
}

impl<'a> Layer<'a> {
    fn get_pixel(&self, x: ImageDimension, y: ImageDimension) -> PixelType {
        if x > self.width {
            panic!("Invalid x");
        }
        if y > self.height {
            panic!("Invalid y");
        }

        let start = (y * self.width) + x;
        let end = start + 1;
        let c = &self.pixels[start..end];
        match c {
            "0" => PixelType::Black,
            "1" => PixelType::White,
            "2" => PixelType::Transparent,
            _ => panic!("Unrecognised pixel type: {}", c),
        }
    }

    fn count_digits(&self, digit: char) -> ImageDimension {
        self.pixels.chars().filter(|c| *c == digit).count()
    }
}

impl<'a> Iterator for LayerIter<'a> {
    type Item = Layer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index = match self.index {
            Some(i) => Some(i + 1),
            None => Some(0),
        };
        let index = self.index.unwrap();
        self.image.get_layer(index)
    }
}

fn render(image: &SpaceImage) {
    for i in 0..image.height {
        for j in 0..image.width {
            for l in 0..image.layer_count() {
                let layer = image.get_layer(l).unwrap();
                match layer.get_pixel(j, i) {
                    PixelType::Black => {
                        print!(" ");
                        break;
                    }
                    PixelType::White => {
                        print!("█");
                        break;
                    }
                    PixelType::Transparent => {
                        continue;
                    }
                }
            }
        }
        println!("");
    }
}

fn input() -> String {
    util::read_file("inputs/day8.txt")
}

fn calculate_day_eight(input: &str) -> (usize, SpaceImage) {
    let image = SpaceImage::new(input, 25, 6);
    let layer = image.layers().min_by_key(|l| l.count_digits('0')).unwrap();
    let calc = layer.count_digits('1') * layer.count_digits('2');
    (calc, image)
}

pub fn run_day_eight() {
    let input = input();
    let (part_1, image) = calculate_day_eight(&input);
    println!("Day 8, Part 1: {}", part_1);
    println!("Day 8, Part 2:");
    render(&image);
}

#[test]
fn example_1() {
    let image = SpaceImage::new("123456789012", 3, 2);
    assert_eq!(2, image.layer_count());
    assert_eq!(2, image.layers().count());
}

#[test]
fn actual_day_8() {
    let input = input();
    let (part_1, _) = calculate_day_eight(&input);
    assert_eq!(2286, part_1);
}
