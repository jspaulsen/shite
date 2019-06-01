use sdl2::rect::Rect;

#[derive(Clone, Debug)]
pub struct Space {
    internal: Rect,
    relative: bool,
}

impl Space {
    pub fn new(x: i32, y: i32, width: u32, height: u32, rel_pos: bool) -> Self {
        Self {
            internal: Rect::new(x, y, width, height),
            relative: rel_pos,
        }
    }

    pub fn is_relative(&self) -> bool {
        self.relative
    }

    pub fn subtract(&self, space: &Self) -> Self {
        Self::new(
            self.internal.x() - space.internal.x(),
            self.internal.y() - space.internal.y(),
            self.internal.width(),
            self.internal.height(),
            false,
        )
    }

    pub fn relative_space(&self, rel_to: &Space) -> Space {
        let rel_top_left = rel_to.internal.top_left();
        let this_top_left = self.internal.top_left();

        Space::new(
            this_top_left.x() + rel_top_left.x(),
            this_top_left.y() + rel_top_left.y(),
            self.internal.width(),
            self.internal.height(),
            true,
        )
    }

    pub fn abs_space(&self, rel_to: &Space) -> Space {
        let rel_top_left = rel_to.internal.top_left();
        let this_top_left = self.internal.top_left();

        if !self.relative {
            Space::new(
                this_top_left.x() - rel_top_left.x(),
                this_top_left.y() - rel_top_left.y(),
                self.internal.width(),
                self.internal.height(),
                false,
            )
        } else {
            self.clone()
        }
    }

    pub fn rect(&self) -> Rect {
        self.internal
    }
}
