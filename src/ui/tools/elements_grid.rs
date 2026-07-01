use crate::math::Vec2;
use crate::ui::tools::UiElement;
use crate::utils::MutSafePtr;


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Alignment {
    Vertical,
    Horizontal,
}

pub struct ElementsGrid {
    position: Vec2,
    size: Vec2,

    elements: Vec<MutSafePtr<dyn UiElement>>,

    /// elements alignment
    pub alignment: Alignment,

    /// limit of elements in sequence.
    /// 0 = no limit
    pub sequence_limit: i32,

    /// offset between each element
    pub spacing: f32,

    /// space outside of elements
    pub border: Vec2,
}

impl UiElement for ElementsGrid {
    fn get_pos(&self) -> Vec2 { self.position }
    fn get_size(&self) -> Vec2 { self.size }

    fn set_pos(&mut self, x: f32, y: f32) { self.position = Vec2{ x, y } }
    fn set_size(&mut self, x: f32, y: f32) { self.size = Vec2{ x, y } }
}

impl ElementsGrid {
    pub fn new(capacity: usize, alignment: Alignment, sequence_limit: i32, spacing: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ZERO,

            elements: Vec::with_capacity(capacity),

            alignment,
            sequence_limit,
            spacing,
            border: Vec2::ZERO,
        }
    }

    pub fn add(&mut self, element: *mut dyn UiElement) {
        self.elements.push(MutSafePtr::from_ptr(element));
    }

    pub fn update(&mut self) {
        if self.elements.len() == 0 { return }

        let start = self.get_pos() + self.border;

        let mut offset = Vec2::ZERO;
        let mut max_size = Vec2::ZERO;
        let mut slice_max_offset_size = self.get_slice_max_offset(0);

        for i in 0..self.elements.len() {
            if (i != 0) && (i % self.sequence_limit as usize == 0) && (self.sequence_limit > 0)
            {
                if self.alignment == Alignment::Horizontal
                {
                    offset.x = 0.0;
                    offset.y += slice_max_offset_size.y + self.spacing;
                }
                else
                {
                    offset.x += slice_max_offset_size.x + self.spacing;
                    offset.y = 0.0;
                }

                slice_max_offset_size = self.get_slice_max_offset(i);
            }

            let mut element = &mut self.elements[i];

            element.set_posv(start + offset);
            let element_size = element.get_size();

            if self.alignment == Alignment::Horizontal {
                offset.x += element_size.x + self.spacing;
            }
            else {
                offset.y += element_size.y + self.spacing;
            }

            let element_final = element.get_final();
            max_size.x = f32::max(max_size.x, element_final.x - start.x);
            max_size.y = f32::max(max_size.y, element_final.y - start.y);
        }

        self.set_sizev(max_size + self.border * 2.0);
    }

    fn get_slice_max_offset(&self, start_index: usize) -> Vec2 {
        let mut max = Vec2::ZERO;

        // check limit range
        let mut end = start_index + self.sequence_limit as usize;

        if end > self.elements.len() {
            end = self.elements.len();
        }

        for i in start_index..end {
            let element_size = self.elements[i].get_size();

            max.x = f32::max(max.x, element_size.x);
            max.y = f32::max(max.y, element_size.y);
        }

        return max;
    }
}
