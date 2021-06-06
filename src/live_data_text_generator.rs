use std::fs::File;
use std::io::Write;

use resol_vbus::{
    chrono::prelude::*,
    DataSet,
    Language,
    Specification,
};


use config::Config;
use error::{Result};


pub struct LiveDataTextGenerator {
    pub spec: Specification,
    pub filename: String,
}


impl LiveDataTextGenerator {
    pub fn from_config(config: &Config) -> Result<LiveDataTextGenerator> {
        let spec_file = config.load_spec_file()?;

        let spec = Specification::from_file(spec_file, Language::En);

        let filename = config.live_data_text_output_filename.clone();

        Ok(LiveDataTextGenerator {
            spec,
            filename,
        })
    }

    pub fn generate(&mut self, orig_data_set: &DataSet, _now: &DateTime<UTC>) -> Result<()> {
        let mut data_set = orig_data_set.clone();

        data_set.sort();

        let mut output = File::create(&self.filename)?;

        for field in self.spec.fields_in_data_set(&data_set) {
            let value = field.fmt_raw_value(false);
            let unit_text = field.field_spec().unit_text.trim();
            let packet_name = &field.packet_spec().name;
            let field_name = &field.field_spec().name;

            write!(output, "{}_{};{};{};{}: {}\n", field.packet_spec().packet_id, field.field_spec().field_id, value, unit_text, packet_name, field_name)?;
        }

        output.flush()?;
        drop(output);

        Ok(())
    }
}
