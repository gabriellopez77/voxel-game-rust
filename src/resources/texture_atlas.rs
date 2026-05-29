use std::path::PathBuf;
use crate::math::Vec2;
use crate::resources::TexCoords;


pub fn create(images_path: &Vec<PathBuf>, width: i32, height: i32) -> (Vec<u8>, Vec<(String, TexCoords)>) {
    let mut root = Node::new(0, 0, width, height);

    let buffer_size = (width * height * 4) as usize;
    let mut images_coords: Vec<(String, TexCoords)> = Vec::with_capacity(images_path.len());
    let mut atlas_pixels: Vec<u8> = vec![0; buffer_size];

    for path in images_path {
        let image = image::open(path).expect("Failed to open image");
        let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();

        if let Some(ref rect) = add_image(&mut root, &image, &mut atlas_pixels, width) {
            let coords = TexCoords::newi(
                rect.x, rect.y,
                rect.x + rect.width,
                rect.y + rect.height
            ).normalized(Vec2::new(width as f32, height as f32));

            images_coords.push((file_name, coords));
        }
        else { println!("Error to Insert: {file_name}") }
    }

    return (atlas_pixels, images_coords);
}

fn add_image(root: &mut Box<Node>, image_info: &image::DynamicImage, atlas_pixels: &mut [u8], atlas_width: i32) -> Option<ImageRect> {
    let node = Node::insert(root, image_info.width() as i32, image_info.height() as i32);

    if let Some(n) = node {

        let rect = n.rect;

        // if image is not rgba then convert it to rgba
        let data = match image_info.as_rgba8() {
            Some(x) => x,
            None => &image_info.to_rgba8()
        };
        
        write_image(&data, rect, atlas_pixels, atlas_width);

        return Some(rect);
    }

    return None;
}

fn write_image(image_pixels: &[u8], rect: ImageRect, atlas_pixels: &mut [u8], atlas_width: i32) {
    let (atlas_prefix, atlas_middle, atlas_suffix) = unsafe { atlas_pixels.align_to_mut::<u32>() };
    let (image_prefix, image_middle, image_suffix) = unsafe { image_pixels.align_to::<u32>() };

    // check if cast is valid
    if !atlas_prefix.is_empty() || !atlas_suffix.is_empty() { panic!("Cannot cast atlas pixels [u8] to [u32]") }
    if !image_prefix.is_empty() || !image_suffix.is_empty() { panic!("Cannot cast images pixels [u8] to [u32]") }


    // copy image pixels to atlas pixels
    for x in 0..rect.width {
        for y in 0..rect.height {
            let image_index = x + rect.width * y;
            let atlas_index = rect.x + x + atlas_width * (rect.y + y);

            atlas_middle[atlas_index as usize] = image_middle[image_index as usize];
        }
    }
}


#[derive(Copy, Clone)]
struct ImageRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

struct Node {
    rect: ImageRect,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    used: bool,
}

impl Node {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Box<Self> {
        Box::new(Self {
            rect: ImageRect { x, y, width, height },
            left: None,
            right: None,
            used: false
        })
    }

    pub fn insert(n: &mut Box<Node>, width: i32, height: i32) -> Option<&Box<Node>>{
        // Se nao e folha
        if n.left.is_some() && n.right.is_some() {
            let node = Self::insert(n.left.as_mut().unwrap(), width, height);

            if node.is_some() { return node; }

            return Self::insert(n.right.as_mut().unwrap(), width, height);
        }

        // ja ocupado
        if n.used { return None }

        // Nao cabe
        if width > n.rect.width || height > n.rect.height {
            return None
        }

        // perfeito
        if width == n.rect.width && height == n.rect.height {
            n.used = true;
            return Some(n);
        }

        // split
        let dw = n.rect.width - width;
        let dh = n.rect.height - height;

        let rect = n.rect;

        if dw > dh {
            n.left = Some(Node::new(rect.x, rect.y, width, rect.height));
            n.right = Some(Node::new(rect.x + width, rect.y, rect.width - width, rect.height));
        }
        else {
            n.left = Some(Node::new(rect.x, rect.y, rect.width, height));
            n.right = Some(Node::new(rect.x, rect.y + height, rect.width, rect.height - height));
        }

        return Self::insert(n.left.as_mut().unwrap(), width, height);
    }
}
