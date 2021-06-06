use std::collections::HashMap;

use resol_vbus::{
    chrono::prelude::*,
    DataSet,
    Language,
    Specification,
};

use sqlite::{
    Connection,
    State,
};

use config::Config;
use error::Result;


pub struct SqliteLogger {
    spec: Specification,
    connection: Connection,
    statement: String,
    fields: Vec<String>,
}


impl SqliteLogger {
    pub fn from_config(config: &Config) -> Result<SqliteLogger> {
        let spec_file = config.load_spec_file()?;

        let spec = Specification::from_file(spec_file, Language::En);

        let connection = sqlite::open(&config.sqlite_filename)?;

        let statement = config.sqlite_statement.clone();

        let fields = config.sqlite_fields.clone();

        Ok(SqliteLogger{
            spec,
            connection,
            statement,
            fields,
        })
    }

    pub fn log(&mut self, data_set: &DataSet, now: &DateTime<UTC>) -> Result<()> {
        let local_now = now.with_timezone(&Local);

        let mut field_map = HashMap::new();

        for field in self.spec.fields_in_data_set(data_set) {
            let key = format!("{}_{}", field.packet_spec().packet_id, field.field_spec().field_id);
            let value = field.raw_value_f64();
            field_map.insert(key, value);
        }

        let mut stmt = self.connection.prepare(&self.statement)?;

        for (idx, field) in self.fields.iter().enumerate() {
            let idx = idx + 1;
            match field.as_str() {
                "UtcDateTime" => stmt.bind(idx, now.to_rfc3339().as_str())?,
                "LocalDateTime" => stmt.bind(idx, local_now.to_rfc3339().as_str())?,
                field => match field_map.get(field) {
                    Some(Some(v)) => stmt.bind(idx, *v)?,
                    _ => stmt.bind(idx, ())?,
                },
            }
        }

        while stmt.next()? != State::Done {
            // repeat
        }

        Ok(())
    }
}
