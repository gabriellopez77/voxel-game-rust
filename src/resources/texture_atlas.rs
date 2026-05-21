use std::{cell::RefCell, path::PathBuf, rc::Rc};
use crate::math::Vec2;
use crate::resources::TexCoords;


pub fn create(images_path: &Vec<PathBuf>, width: i32, height: i32) -> (Vec<u8>, Vec<(String, TexCoords)>) {
    let root = Node::new(0, 0, width, height);

    let buffer_size = (width * height * 4) as usize;
    let mut images_coords: Vec<(String, TexCoords)> = Vec::with_capacity(images_path.len());
    let mut atlas_pixels: Vec<u8> = vec![0; buffer_size];
    
    for path in images_path {
        let image = image::open(path).expect("Failed to open image");
        let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();

        if let Some(ref rect) = add_image(root.clone(), &image, &mut atlas_pixels, width) {
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

fn add_image(root: Rc<RefCell<Node>>, image_info: &image::DynamicImage, atlas_pixels: &mut [u8], atlas_width: i32) -> Option<ImageRect> {
    let node = Node::insert(root.clone(), image_info.width() as i32, image_info.height() as i32);

    if let Some(n) = node {

        let rect = n.borrow().rect;

        // if image is not rgba then convert it to rgba
        let data = match image_info.as_rgba8() {
            Some(x) => x,
            None => &image_info.to_rgba8()
        };

        //let data = image_info.to_rgba8().into_raw();
        //write_image(data.as_raw(), rect, atlas_pixels, atlas_width);
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
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    used: bool,
}

impl Node {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            rect: ImageRect { x, y, width, height },
            left: None,
            right: None,
            used: false
        }))
    }

    pub fn insert(n: Rc<RefCell<Node>>, width: i32, height: i32) -> Option<Rc<RefCell<Node>>>{
        let it = &mut n.borrow_mut();

        // Se nao e folha
        if it.left.is_some() && it.right.is_some() {
            let node = Self::insert(it.left.as_ref().unwrap().clone(), width, height);
            
            if node.is_some() { return node; }
            
            return Self::insert(it.right.as_ref().unwrap().clone(), width, height);
        }

        // ja ocupado
        if it.used { return None }

        // Nao cabe
        if width > it.rect.width || height > it.rect.height {
            return None
        }

        // perfeito
        if width == it.rect.width && height == it.rect.height {
            it.used = true;
            return Some(n.clone());
        }

        // split
        let dw = it.rect.width - width;
        let dh = it.rect.height - height;

        let rect = it.rect;

        if dw > dh {
            it.left = Some(Node::new(rect.x, rect.y, width, rect.height));
            it.right = Some(Node::new(rect.x + width, rect.y, rect.width - width, rect.height));
        }
        else
        {
            it.left = Some(Node::new(rect.x, rect.y, rect.width, height));
            it.right = Some(Node::new(rect.x, rect.y + height, rect.width, rect.height - height));
        }

        return Self::insert(it.left.as_ref().unwrap().clone(), width, height);
    }
}