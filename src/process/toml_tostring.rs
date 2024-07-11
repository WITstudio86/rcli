use std::fs::File;

use anyhow::Result;
use csv::{Reader, StringRecord};

pub fn to_toml_string(header: StringRecord, reader: &mut Reader<File>) -> Result<String> {
    let mut ret_vec = Vec::with_capacity(128);
    for record in reader.records() {
        let record = record?;
        let mut zipped = header.iter().map(|i| i.to_string()).zip(record.iter());
        if let Some(t) = zipped.next() {
            let (_, name) = t;
            ret_vec.push(format!("[\"{}\"]", name));
            for (mut key, value) in zipped {
                // 如果 value 包含空格
                if key.contains(' ') {
                    // 替换为下划线避免报错
                    key = key.replace(' ', "_");
                }
                ret_vec.push(format!("{} = \"{}\"", key, value));
            }
        }
    }
    Ok(ret_vec.join("\n"))
}
