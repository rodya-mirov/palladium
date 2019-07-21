use super::*;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Image,
};

use game_state::Loadable;

use numerics::force_max;

pub struct CenteredVerticalImagesRenderer<'a> {
    pub window: &'a mut Window,
    pub images: &'a mut Vec<&'a mut Asset<Image>>,
    pub bg_color: Color,
    // applied on top/bottom and left/right
    pub outside_padding: Vector,
    // applied between images
    pub internal_padding: Vector,
}

impl<'a, 'b> System<'a> for CenteredVerticalImagesRenderer<'a> {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        if !self.images.is_loaded().unwrap_or(false) {
            return;
        }

        let window = &mut self.window;
        let images = &mut self.images;
        let internal_padding = self.internal_padding;
        let outside_padding = self.outside_padding;

        let mut total_height = outside_padding.y * 2.0 + (internal_padding.y * (images.len() - 1) as f32);
        let mut max_width = 0.0;

        for image in images.iter_mut() {
            image
                .execute(|image| {
                    let size = image.area().size();
                    total_height += size.y;
                    max_width = force_max(max_width, size.x);
                    Ok(())
                })
                .expect("Images should not error");
        }

        let total_width = max_width + (outside_padding.x * 2.0);

        let screen_size = window.screen_size();
        let padded_box_size = Vector::new(total_width, total_height);

        let padded_rect = Rectangle {
            pos: (screen_size - padded_box_size) / 2.0,
            size: padded_box_size,
        };

        window.draw(&padded_rect, Col(self.bg_color));

        let mut render_pos = padded_rect.pos + outside_padding;

        for image in images.iter_mut() {
            image
                .execute(|image| {
                    let size = image.area().size();
                    window.draw(&Rectangle { pos: render_pos, size }, Img(image));
                    render_pos.y += size.y + internal_padding.y;
                    Ok(())
                })
                .expect("Don't error on rendering image, thanks");
        }
    }
}
