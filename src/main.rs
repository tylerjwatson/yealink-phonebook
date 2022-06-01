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

use crate::vcard::{get_contacts, prop_value};
use minidom::Element;
use rocket::{fairing::AdHoc, response::content::RawXml, serde::Deserialize, State};
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
    let contact_url: Url = config.vcf_url.parse().expect("Invalid VCF Url");

    let cards = get_contacts(contact_url).await.expect("Cannot get cards");

    let mut builder = Element::builder("IPPhoneDirectory", "");

    for card in cards {
        let name = prop_value(&card.properties, "FN");
        let tel = prop_value(&card.properties, "TEL");

        if tel == None {
            continue;
        }

        builder = builder.append(
            Element::builder("DirectoryEntry", "")
                .append(Element::builder("Name", "").append(name.unwrap()))
                .append(Element::builder("Telephone", "").append(tel.unwrap())),
        );
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
