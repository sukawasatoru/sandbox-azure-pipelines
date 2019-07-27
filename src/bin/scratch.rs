#![allow(unused_imports, dead_code)]

use std::{
    fmt,
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
};

use directories;
use dotenv;
use env_logger;
use failure::{bail, ensure};
use log::info;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_derive::{Deserialize, Serialize};
use toml;

use rust_myscript::myscript::prelude::*;

#[derive(Debug, Serialize)]
struct DeSample {
    val1: String,
    #[serde(skip_serializing)]
    val_opt_1: String,
    #[serde(skip_serializing)]
    val_opt_2: String,
    val2: String,
    val3: i32,
}

const DE_SAMPLE_FIELDS: &'static [&'static str] = &["val1", "val2", "val3"];

enum DeSampleField {
    Val1,
    Val2,
    Val3,
}

impl DeSample {
    fn new(val1: &str, val2: &str, val3: i32) -> DeSample {
        let spl = val1.to_string();
        let spl = spl.split('/').collect::<Vec<_>>();
        DeSample {
            val1: val1.to_string(),
            val_opt_1: spl[0].to_string(),
            val_opt_2: spl[1].to_string(),
            val2: val2.to_string(),
            val3,
        }
    }
}

impl<'de> Deserialize<'de> for DeSample {
    fn deserialize<D>(deserializer: D) -> Result<DeSample, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("DeSample", DE_SAMPLE_FIELDS, DeSampleVisitor)
    }
}

impl<'de> Deserialize<'de> for DeSampleField {
    fn deserialize<D>(deserializer: D) -> Result<DeSampleField, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DeSampleFieldVisitor;

        impl<'de> Visitor<'de> for DeSampleFieldVisitor {
            type Value = DeSampleField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("DeSample struct fields")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "val1" => Ok(DeSampleField::Val1),
                    "val2" => Ok(DeSampleField::Val2),
                    "val3" => Ok(DeSampleField::Val3),
                    _ => Err(de::Error::unknown_field(v, DE_SAMPLE_FIELDS)),
                }
            }
        }
        deserializer.deserialize_identifier(DeSampleFieldVisitor)
    }
}

struct DeSampleVisitor;

impl<'de> Visitor<'de> for DeSampleVisitor {
    type Value = DeSample;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a DeSample struct")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<DeSample, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let val1 = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let val2 = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let val3 = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        Ok(DeSample::new(val1, val2, val3))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut val1 = None;
        let mut val2 = None;
        let mut val3 = None;
        while let Some(key) = map.next_key()? {
            match key {
                "val1" => {
                    if val1.is_some() {
                        return Err(de::Error::duplicate_field("val1"));
                    }
                    val1 = Some(map.next_value()?);
                }
                "val2" => {
                    if val2.is_some() {
                        return Err(de::Error::duplicate_field("val2"));
                    }
                    val2 = Some(map.next_value()?);
                }
                "val3" => {
                    if val3.is_some() {
                        return Err(de::Error::duplicate_field("val3"));
                    }
                    val3 = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, DE_SAMPLE_FIELDS)),
            }
        }
        let val1 = val1.ok_or_else(|| de::Error::missing_field("val1"))?;
        let val2 = val2.ok_or_else(|| de::Error::missing_field("val2"))?;
        let val3 = val3.ok_or_else(|| de::Error::missing_field("val3"))?;
        Ok(DeSample::new(val1, val2, val3))
    }
}

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Hello");
    let sample: DeSample = toml::from_str(
        r#"val1 = "valval1/child"
val2 = "valval2"
val3 = 256
"#,
    )?;
    println!("{:?}", sample);
    info!("Bye");

    Ok(())
}

fn fail_func() -> Fallible<()> {
    bail!("TODO")
}

fn todo(message: &str) -> Fallible<()> {
    ensure!(false, "TODO: {}", message);
    Ok(())
}

fn io_err() -> Fallible<()> {
    std::fs::File::open("/hoge")?;
    Ok(())
}
