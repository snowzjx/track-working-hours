use rocket::request::FromForm;
use rocket::request::FormItems;

use std::collections::HashMap;

pub struct TrackForm<'f> {
    pub map: HashMap<&'f str, &'f str>,
}

impl<'f> FromForm <'f> for TrackForm<'f> {
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, __strict: bool) -> Result<Self, Self::Error> {
        let mut track_form = TrackForm {
            map: HashMap::new(),
        };

        for item in items {
            let key = item.key;
            let value = item.value;
            track_form.map.insert(key, value);
        }
        
        Ok(track_form)
    }
}