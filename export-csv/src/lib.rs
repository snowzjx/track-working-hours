use std::error::Error;
use data::*;

pub fn export_csv<'a>() -> Result<(), Box<dyn Error>> {
    
    let trackings = select_grouped_trackings()?;

    let mut writer = csv::Writer::from_path("exported.csv")?;

    writer.write_record(&["项目名称", "状态", "额外信息", "参与人", "累计时间"])?;
    for values in trackings.values() {
        for value in values {
            writer.write_field(&value.0.name)?;
            writer.write_field(&value.0.status)?;
            writer.write_field(&value.0.info)?;
            writer.write_field(&value.1.display_name)?;
            match value.2 {
                Some(value) => writer.write_field(value.to_string())?,
                None => writer.write_field(0.to_string())?
            }
            writer.write_record(None::<&[u8]>)?;
        }
    }
    writer.flush()?;
    Ok(())
}
