use std::collections::HashMap;
use indexmap::IndexMap;
use plotters::prelude::*;
use crate::animals::Species;

pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub const HERBIVORE: Colour = Colour { r: 132, g: 191, b: 161 };
    pub const CARNIVORE: Colour = Colour { r: 242, g: 195, b: 143 };
    pub const BACKGROUND: Colour = Colour { r: 251, g: 250, b: 245 };
    pub const AXIS: Colour = Colour { r: 0, g: 0, b: 0 };

    pub fn colour(&self) -> RGBColor {
        RGBColor(self.r, self.g, self.b)
    }
}

pub struct Graphics {
    pub path: &'static str,
}

impl Graphics {
    pub fn graph(&self, data: &IndexMap<Species, Vec<u32>>) {
        let root = BitMapBackend::new(&self.path, (1024, 768)).into_drawing_area();
        root.fill(&Colour::BACKGROUND.colour()).expect("Failed to fill the drawing area");

        let max_x = data.values().map(|v| v.len()).max().unwrap_or(10) - 1;
        let max_y = data.values().flat_map(|v| v.iter()).max().unwrap_or(&0) + 10;

        let mut chart = ChartBuilder::on(&root)
            .caption("Population dynamics", ("monospace", 40))
            .margin(5)
            .set_all_label_area_size(40)
            .build_cartesian_2d(0..max_x, 0..max_y)
            .expect("Failed to build the chart");

        chart.configure_mesh()
            .x_labels(10) // Number of labels on the x-axis
            .y_labels(10) // Number of labels on the y-axis
            .disable_x_mesh() // Disable grid lines on the x-axis
            .disable_y_mesh()
            .label_style(("monospace", 15).into_font())// Disable grid lines on the y-axis
            .draw()
            .expect("Failed to draw the axes");


        for (species, data) in data.iter() {
            let style = ShapeStyle::from(
                &match species {
                    Species::Herbivore => Colour::HERBIVORE.colour(),
                    Species::Carnivore => Colour::CARNIVORE.colour(),
                }
            ).stroke_width(2);

            chart.draw_series(
                LineSeries::new(
                    data.iter().enumerate()
                        .map(|(i, v)| (i, *v)),
                    style
                ))
                .expect("Failed to draw the series")
                .label(species.to_string())
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style));
        }
        chart.configure_series_labels()
            .border_style(&Colour::AXIS.colour())
            .background_style(&Colour::BACKGROUND.colour())
            .label_font(("monospace", 20))
            .draw()
            .expect("Failed to configure the labels");
    }
}
