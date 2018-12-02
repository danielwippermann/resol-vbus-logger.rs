use std::io::Write;

use resol_vbus::{
    chrono::prelude::*,
    id_hash,
    DataSet,
    Language,
    Specification,
    SpecificationFile,
};


use config::Config;
use error::{Result};
use timestamp_file_writer::TimestampFileWriter;


pub struct CsvGenerator {
    pub spec: Specification,
    pub file_writer: TimestampFileWriter<Local>,
    pub id_hash: Option<u64>,
}


impl CsvGenerator {
    pub fn from_config(config: &Config) -> Result<CsvGenerator> {
        let spec_file = SpecificationFile::new_default();

        let spec = Specification::from_file(spec_file, Language::De);

        let file_writer = TimestampFileWriter::new(config.csv_output_filename_pattern.clone(), Local::now());

        Ok(CsvGenerator {
            spec,
            file_writer,
            id_hash: None,
        })
    }

    pub fn generate(&mut self, orig_data_set: &DataSet, now: &DateTime<UTC>) -> Result<()> {
        let mut data_set = orig_data_set.clone();

        data_set.sort();

        let local_now = now.with_timezone(&Local);

        let output = &mut self.file_writer;

        let is_new_file = output.set_timestamp(local_now)?;

        let current_id_hash = id_hash(&data_set);

        let id_hash_differs = if let Some(prev_id_hash) = self.id_hash {
            current_id_hash != prev_id_hash
        } else {
            true
        };

        self.id_hash = Some(current_id_hash);

        let need_header = if is_new_file {
            true
        } else if id_hash_differs {
            true
        } else {
            false
        };

        if need_header {
            debug!("Needs header: is new file = {}, ID hash differs = {}, filename = {}", is_new_file, id_hash_differs, output.filename().unwrap());

            write!(output, "Datum")?;

            for field in self.spec.fields_in_data_set(&data_set) {
                let name = &field.field_spec().name;
                let unit_text = field.field_spec().unit_text.trim();
                if unit_text.len() > 0 {
                    write!(output, "\t{} [{}]", name, unit_text)?;
                } else {
                    write!(output, "\t{}", name)?;
                }
            }

            write!(output, "\n")?;
        }

        write!(output, "{}", local_now.format("%Y.%m.%d %H:%M:%S"))?;

        for field in self.spec.fields_in_data_set(&data_set) {
            write!(output, "\t{}", field.fmt_raw_value(false))?;
        }

        write!(output, "\n")?;

        output.flush()?;

        Ok(())
    }
}