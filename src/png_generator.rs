use std::collections::HashMap;

use image::{DynamicImage, Rgba};

use imageproc::drawing::draw_text_mut;

use resol_vbus::{
    chrono::prelude::*,
    DataSet,
    Language,
    Specification,
    SpecificationFile,
};

use rusttype::{Font, Scale};


use config::Config;

use error::{Error, Result};


pub struct PngGenerator<'a> {
    pub spec: Specification,

    pub img: Option<DynamicImage>,
    pub font: Font<'a>,
    pub png_output_filename: String,
}


impl<'a> PngGenerator<'a> {
    pub fn from_config(config: &Config) -> Result<PngGenerator<'a>> {
        let spec_file = SpecificationFile::new_default();

        let spec = Specification::from_file(spec_file, Language::De);

        let img = if config.png_tick_interval > 0 {
            Some(image::open(&config.png_input_filename)?)
        } else {
            None
        };

        let font = Vec::from(include_bytes!("../Roboto-Regular.ttf") as &[u8]);

        let font = match Font::try_from_vec(font) {
            Some(font) => font,
            None => return Err("Unable to parse font".into()),
        };

        let png_output_filename = config.png_output_filename.clone();

        Ok(PngGenerator {
            spec,
            img,
            font,
            png_output_filename,
        })
    }

    pub fn generate(&self, data_set: &DataSet, now: &DateTime<UTC>) -> Result<()> {
        let local_now = now.with_timezone(&Local);

        let mut field_map = HashMap::new();

        for field in self.spec.fields_in_data_set(data_set) {
            let key = format!("{}_{}", field.packet_spec().packet_id, field.field_spec().field_id);
            let value = format!("{}", field.fmt_raw_value(true));
            field_map.insert(key, value);
        }

        let mut img = match self.img {
            Some(ref img) => img.clone(),
            None => return Err(Error::from("No image loaded")),
        };

        let default_str = "---".to_string();

        // let scale = Scale { x: 26.0, y: 26.0 };
        // draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 495, 11, scale, &self.font, "Gesamtertrag: ");

        // let scale = Scale { x: 26.0, y: 26.0 };
        // draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 15, 7, scale, &self.font, "Messzeitpunkt:");

        // let scale = Scale { x: 20.0, y: 20.0 };
        // draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 742, 13, scale, &self.font, "seit 29.08.2015");

        // let scale = Scale { x: 22.0, y: 22.0 };
        // draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 621, 487, scale, &self.font, "S4:");

        // let scale = Scale { x: 20.0, y: 20.0 };
        // draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 765, 303, scale, &self.font, "Soll");

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_7210_6521_10_0100_002_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 526, 88, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_7210_6521_10_0100_000_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 753, 374, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_0010_7210_10_0100_000_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 261, 64, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_0010_7210_10_0100_010_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 374, 174, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_0010_7210_10_0100_004_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 357, 599, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_7210_6521_10_0100_004_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 269, 332, scale, &self.font, value);

        let scale = Scale { x: 24.0, y: 24.0 };
        let value = format!("{}", local_now.format("%d.%m.%Y %H:%M:%S"));
        // draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 144, 8, scale, &self.font, &value);
        draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 164, 12, scale, &self.font, &value);

        let scale = Scale { x: 24.0, y: 24.0 };
        let value = field_map.get("00_0010_7210_10_0100_024_2_0").unwrap_or(&default_str);
        // draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 620, 12, scale, &self.font, value);
        draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 620, 15, scale, &self.font, value);

        let scale = Scale { x: 24.0, y: 24.0 };
        let value = field_map.get("00_6521_7210_10_0200_032_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 180, 717, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_6521_7210_10_0200_000_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 715, 352, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_6521_7210_10_0200_024_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 197, 378, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_0010_7210_10_0100_020_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 498, 276, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_0010_7210_10_0100_021_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 331, 415, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_0010_7210_10_0100_023_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 271, 421, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_6521_7210_10_0200_032_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 654, 612, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_0010_7210_10_0100_002_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([128u8, 0u8, 0u8, 255u8]), 457, 329, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_0010_7210_10_0100_006_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([128u8, 0u8, 0u8, 255u8]), 386, 329, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_0010_7210_10_0100_008_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([128u8, 0u8, 0u8, 255u8]), 382, 396, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_6521_7210_10_0200_008_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 671, 317, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_6521_7210_10_0200_016_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 667, 359, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_7210_6521_10_0100_008_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 523, 143, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_0010_7210_10_0100_018_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 496, 291, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_0010_7210_10_0100_004_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 376, 523, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_6521_7210_10_0200_032_1_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 188, 450, scale, &self.font, value);

        let scale = Scale { x: 22.0, y: 22.0 };
        let value = field_map.get("00_7210_6521_10_0100_006_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([255u8, 0u8, 0u8, 255u8]), 650, 487, scale, &self.font, value);

        let scale = Scale { x: 20.0, y: 20.0 };
        let value = field_map.get("00_0010_7211_10_0100_000_2_0").unwrap_or(&default_str);
        draw_text_mut(&mut img, Rgba([0u8, 0u8, 0u8, 255u8]), 762, 321, scale, &self.font, value);

        img.save(&self.png_output_filename)?;

        Ok(())
    }
}
