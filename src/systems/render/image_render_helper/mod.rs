use super::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[allow(dead_code)] // we want to support all these corners for the future
pub enum Corner {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[allow(dead_code)] // we want to support all these corners for the future
pub enum Alignment {
    Vertical,
    Horizontal,
}

pub fn render_image_corner(window: &mut Window, image: &Image, offset: Vector, corner: Corner) {
    render_images_corner(window, &[image], offset, Vector::new(0, 0), corner, Alignment::Horizontal);
}

pub fn render_images_corner(
    window: &mut Window,
    images: &[&Image],
    offset: Vector,
    interior_padding: Vector,
    corner: Corner,
    alignment: Alignment,
) {
    let screen_size = window.screen_size();
    let images_size = get_images_size(images, interior_padding, alignment);

    let mut draw_pos = get_draw_pos(screen_size, images_size, offset, corner);

    for image in images {
        window.draw(&image.area().translate(draw_pos), Img(image));
        match alignment {
            Alignment::Horizontal => {
                draw_pos.x += image.area().size.x + interior_padding.x;
            }
            Alignment::Vertical => {
                draw_pos.y += image.area().size.y + interior_padding.y;
            }
        }
    }
}

fn get_draw_pos(screen_size: Vector, images_size: Vector, offset: Vector, corner: Corner) -> Vector {
    match corner {
        Corner::UpperLeft => offset,
        Corner::UpperRight => {
            let x = screen_size.x - images_size.x - offset.x;
            let y = offset.y;
            Vector::new(x, y)
        }
        Corner::LowerLeft => {
            let x = offset.x;
            let y = screen_size.y - images_size.y - offset.y;
            Vector::new(x, y)
        }
        Corner::LowerRight => screen_size - offset - images_size,
    }
}

fn get_images_size(images: &[&Image], interior_padding: Vector, alignment: Alignment) -> Vector {
    if images.is_empty() {
        return Vector::new(0, 0);
    }

    match alignment {
        Alignment::Horizontal => {
            let mut total_width = interior_padding.x * (images.len() as f32 - 1.0);
            let mut max_height = 0.0;

            for img in images {
                let area = img.area().size;
                total_width += area.x;
                max_height = force_max(max_height, area.y);
            }

            Vector::new(total_width, max_height)
        }
        Alignment::Vertical => {
            let mut max_width = 0.0;
            let mut total_height = interior_padding.y * (images.len() as f32 - 1.0);

            for img in images {
                let area = img.area().size;
                max_width = force_max(max_width, area.x);
                total_height += area.y;
            }

            Vector::new(max_width, total_height)
        }
    }
}
