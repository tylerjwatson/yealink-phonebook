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

mod vcard;

use crate::vcard::get_contacts;
use minidom::Element;
use rocket::{serde::Deserialize, fairing::AdHoc, State, response::content::RawXml};
use url::Url;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    vcf_url: String,
}

#[get("/")]
async fn index(config: &State<Config>) -> RawXml<String> {
    let contact_url: Url = config.vcf_url
        .parse()
        .expect("Invalid VCF Url");

    let cards = get_contacts(contact_url).await.expect("Cannot get cards");

    let mut builder = Element::builder("IPPhoneDirectory", "");

    for card in cards {
        let mut entry = Element::builder("DirectoryEntry", "");

        if let Some(name) = card.properties.iter().find(|i| i.name == "FN") {
            let name_element = Element::builder("Name", "")
                .append(match &name.value {
                    Some(v) => v,
                    None => "Unknown",
                })
                .build();
            entry = entry.append(name_element);
        };

        if let Some(tel) = card.properties.iter().find(|i| i.name == "TEL") {
            let tel_element = Element::builder("Telephone", "")
                .append(match &tel.value {
                    Some(v) => v,
                    None => "Unknown",
                })
                .build();
            entry = entry.append(tel_element);
        };

        builder = builder.append(entry);
    }

    RawXml(String::from(&builder.build()))
}

#[launch]
fn launch() -> rocket::Rocket<rocket::Build> {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .attach(AdHoc::config::<Config>());

    let figment = rocket.figment();

    let config = figment.extract::<Config>().expect("A VCF URL is required.");
    
    println!("yealink-phonebook using VCF URL {}", &config.vcf_url);

    rocket
}
