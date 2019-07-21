//! Render system for the controls pane. Fundamentally just a "render this at the upper right
//! corner of the screen"

use super::*;

use std::collections::HashMap;

use quicksilver::graphics::Image;

use components::{Breathes, Hackable, HasPosition, Player, Talkable};
use resources::{GameClock, GameMapDisplayOptions, GameMapRenderParams};

use image_render_helper::{render_images_corner, Alignment, Corner};
use numerics::force_max;

#[derive(SystemData)]
pub struct ControlsRendererSystemData<'a> {
    player: ReadStorage<'a, Player>,
    breathes: ReadStorage<'a, Breathes>,
    game_clock: Read<'a, GameClock>,

    has_position: ReadStorage<'a, HasPosition>,
    hackable: ReadStorage<'a, Hackable>,
    talkable: ReadStorage<'a, Talkable>,
    entities: Entities<'a>,

    game_map_render_params: Read<'a, GameMapRenderParams>,
    game_map_display_options: Read<'a, GameMapDisplayOptions>,
}

/// Noop system for setup
pub struct ControlsRendererSetup;

impl<'a> System<'a> for ControlsRendererSetup {
    type SystemData = ControlsRendererSystemData<'a>;

    fn run(&mut self, _data: Self::SystemData) {}
}

/// System which renders all the controls stuff
pub struct ControlsRenderer<'a> {
    pub window: &'a mut Window,
    pub controls_image: &'a mut game_state::ControlsImages,
    pub tileset: &'a mut Asset<HashMap<char, Image>>,
}

impl<'a, 'b> ControlsRenderer<'b> {
    fn render_controls_image(&mut self, data: &mut ControlsRendererSystemData<'a>) {
        let player_position = (&data.player, &data.has_position)
            .join()
            .map(|(_, hp)| hp.position)
            .next()
            .expect("Player should have position");

        let adj_positions = super::super::direct_neighbors(player_position);

        let window = &mut self.window;
        let (hack, repair, talk) = (
            &mut self.controls_image.hack,
            &mut self.controls_image.repair,
            &mut self.controls_image.talk,
        );

        hack.execute(|hack| {
            repair.execute(|repair| {
                talk.execute(|talk| {
                    let mut is_hack = false;
                    let mut is_talk = false;
                    let is_repair = false;

                    for (entity, hp) in (&data.entities, &data.has_position).join() {
                        if adj_positions.iter().any(|ap| *ap == hp.position) {
                            if data.hackable.contains(entity) {
                                is_hack = true;
                            }
                            if data.talkable.contains(entity) {
                                is_talk = true;
                            }
                        }
                    }

                    let mut images: Vec<&Image> = Vec::new();
                    if is_hack {
                        images.push(&hack);
                    }
                    if is_talk {
                        images.push(&talk);
                    }
                    if is_repair {
                        images.push(&repair);
                    }

                    let offset = data.game_map_render_params.controls_image_offset_px;

                    image_render_helper::render_images_corner(
                        window,
                        &images,
                        offset,
                        Vector::new(5.0, 5.0),
                        Corner::UpperRight,
                        Alignment::Vertical,
                    );

                    Ok(())
                })
            })
        })
        .expect("Rendering should be successful");
    }
}

impl<'a, 'b> System<'a> for ControlsRenderer<'b> {
    type SystemData = ControlsRendererSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if !data.game_map_display_options.display_controls_pane {
            return;
        }

        self.render_controls_image(&mut data);

        let window = &mut self.window;

        // render clock
        self.tileset
            .execute(|tileset| {
                let game_clock = &data.game_clock;

                let time_string = format!("{:02}:{:02}:{:02}", game_clock.hours, game_clock.minutes, game_clock.seconds);

                let mut total_width = 0.0;
                let mut max_height = 0.0;

                let to_render: Vec<&Image> = time_string
                    .chars()
                    .map(|c| {
                        tileset
                            .get(&c)
                            .unwrap_or_else(|| panic!("Should have defined a tileset item for {}", c))
                    })
                    .map(|img| {
                        let area = img.area().size();
                        max_height = force_max(max_height, area.y);
                        total_width += area.x;
                        img
                    })
                    .collect();

                render_images_corner(
                    window,
                    &to_render,
                    data.game_map_render_params.controls_image_offset_px,
                    Vector::new(0, 0),
                    Corner::UpperLeft,
                    Alignment::Horizontal,
                );

                Ok(())
            })
            .expect("Rendering should work");

        // Render O2 meter (cheap for now)
        self.tileset
            .execute(|tileset| {
                let player_breathe = (&data.player, &data.breathes).join().next();
                if player_breathe.is_none() {
                    return Ok(());
                }

                let breathes = player_breathe.unwrap().1;
                let (capacity, contents) = (breathes.capacity, breathes.contents);
                if contents >= capacity {
                    return Ok(());
                }

                let air_perc = 100.0 * ((contents as f32) / (capacity as f32));
                let air_meter_str = format!("Oxygen: {:.0}%", air_perc);

                let mut total_width = 0.0;
                let mut max_height = 0.0;

                let to_render: Vec<&Image> = air_meter_str
                    .chars()
                    .map(|c| {
                        tileset
                            .get(&c)
                            .unwrap_or_else(|| panic!("Should have defined a tileset item for {}", c))
                    })
                    .map(|img| {
                        let area = img.area().size();
                        max_height = force_max(max_height, area.y);
                        total_width += area.x;
                        img
                    })
                    .collect();

                render_images_corner(
                    window,
                    &to_render,
                    data.game_map_render_params.oxygen_meter_offset_px,
                    Vector::new(0, 0),
                    Corner::UpperLeft,
                    Alignment::Horizontal,
                );

                Ok(())
            })
            .expect("Rendering O2 meter should work");
    }
}
