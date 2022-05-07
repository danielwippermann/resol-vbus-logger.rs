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

use crate::{
    config::Config,
    error::{Error, Result},
};


enum Mode {
    Relational {
        datasets_table: String,
        fields_table: String,
    },
    Tabular {
        statement: String,
        fields: Vec<String>,
    },
}

pub struct SqliteLogger {
    spec: Specification,
    connection: Connection,
    mode: Mode,
}


impl SqliteLogger {
    pub fn from_config(config: &Config) -> Result<SqliteLogger> {
        let spec_file = config.load_spec_file()?;

        let spec = Specification::from_file(spec_file, Language::En);

        let connection = sqlite::open(&config.sqlite_filename)?;

        let mode = match (&config.sqlite_datasets_table, &config.sqlite_fields_table, &config.sqlite_statement, &config.sqlite_fields) {
            (Some(datasets_table), Some(fields_table), None, None) => {
                let stmt = format!("CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY, timestamp TEXT); CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY, dataset_id INTEGER, packet_field_id TEXT, value REAL)", datasets_table, fields_table);
                connection.execute(stmt)?;

                Mode::Relational {
                    datasets_table: datasets_table.clone(),
                    fields_table: fields_table.clone(),
                }
            },
            (None, None, Some(statement), Some(fields)) => Mode::Tabular {
                statement: statement.clone(),
                fields: fields.clone(),
            },
            _ => return Err(Error::from("Unsupported combination of SQLlite logger configuration")),
        };

        Ok(SqliteLogger{
            spec,
            connection,
            mode,
        })
    }

    pub fn log(&mut self, data_set: &DataSet, now: &DateTime<UTC>) -> Result<()> {
        let local_now = now.with_timezone(&Local);

        let mut field_map = HashMap::new();

        match &self.mode {
            Mode::Relational { datasets_table, fields_table } => {
                let stmt = format!("INSERT INTO {} (timestamp) VALUES (?)", datasets_table);
                let mut stmt = self.connection.prepare(stmt)?;
                stmt.bind(1, now.to_rfc3339().as_str())?;
                while stmt.next()? != State::Done {
                    // repeat
                }

                let mut stmt = self.connection.prepare("SELECT last_insert_rowid()")?;
                if stmt.next()? != State::Row {
                    return Err(Error::from("Expected statement to return row"));
                }

                let dataset_id = stmt.read::<i64>(0)?;

                while stmt.next()? != State::Done {
                    // repeat
                }

                let stmt_string = format!("INSERT INTO {} (dataset_id, packet_field_id, value) VALUES (?, ?, ?)", fields_table);

                for field in self.spec.fields_in_data_set(data_set) {
                    if let Some(raw_value) = field.raw_value_f64() {
                        let packet_field_id = format!("{}_{}", field.packet_spec().packet_id, field.field_spec().field_id);

                        let mut stmt = self.connection.prepare(&stmt_string)?;
                        stmt.bind(1, dataset_id)?;
                        stmt.bind(2, packet_field_id.as_str())?;
                        stmt.bind(3, raw_value)?;

                        while stmt.next()? != State::Done {
                            // repeat
                        }
                    }
                }
            }
            Mode::Tabular { statement, fields } => {
                for field in self.spec.fields_in_data_set(data_set) {
                    let key = format!("{}_{}", field.packet_spec().packet_id, field.field_spec().field_id);
                    let value = field.raw_value_f64();
                    field_map.insert(key, value);
                }

                let mut stmt = self.connection.prepare(&statement)?;

                for (idx, field) in fields.iter().enumerate() {
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
            },
        }

        Ok(())
    }
}
