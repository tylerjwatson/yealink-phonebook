use std::{error::Error, io::BufReader};

use ical::parser::vcard::component::VcardContact;
use url::Url;

pub async fn get_contacts(url: Url) -> Result<Vec<VcardContact>, Box<dyn Error>> {
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;

    let parser = ical::VcardParser::new(
        BufReader::new(
            text.as_bytes()
        )
    );

    let mut card_array: Vec<VcardContact> = Vec::new();

    for card in parser {
        if let Ok(card) = card {
            card_array.push(card);
        }
    }

    Ok(card_array)
}
