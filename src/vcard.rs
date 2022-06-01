/*
MIT License

Copyright (c) 2022 Tyler Watson <tyler@tw.id.au>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::{error::Error, io::BufReader};
use ical::{parser::vcard::component::VcardContact, property::Property};
use url::Url;

pub fn prop_value<'a>(props: &'a Vec<Property>, name: &str) -> Option<&'a str> {
    match props.iter().find(|i| i.name.contains(name)) {
        Some(prop) => match &prop.value {
            Some(v) => Some(v),
            None => None
        },
        None => None
    }
}

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
