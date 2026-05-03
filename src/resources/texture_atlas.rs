use std::{cell::RefCell, rc::Rc};

pub struct TextureAtlas {
    
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
    used: bool
}

impl Node {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            rect: ImageRect { x, y, width, height },
            left: None,
            right: None,
            used: false
        }
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
            it.left = Some(Rc::new(RefCell::new(Node::new(rect.x, rect.y, width, rect.height))));
            it.right = Some(Rc::new(RefCell::new(Node::new(rect.x + width, rect.y, rect.width - width, rect.height))));
        }
        else
        {
            it.left = Some(Rc::new(RefCell::new(Node::new(rect.x, rect.y, rect.width, height))));
            it.right = Some(Rc::new(RefCell::new(Node::new(rect.x, rect.y + height, rect.width, rect.height - height))));
        }

        return Self::insert(it.left.as_ref().unwrap().clone(), width, height);
    }
}