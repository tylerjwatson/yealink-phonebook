# Yealink Phonebook

Yealink Phonebook is a small server that will download a .vcf (vCard) file and convert the contacts in it to a format that is compatible with Yealink Remote Phonebook on-the-fly.

The contacts in the specified vCard will then appear in the Remote Phonebook on the device.

It is useful, for example, to keep your your office phones in sync with your Nextcloud contact list.

Yealink Phonebook has been tested and working with **SIP-T46G** VoIP phones, and **Nextcloud 24.0.1**.

## Installing Yealink Phonebook Locally

```bash
cargo install yealink-phonebook
```

## Configuring

Yealink Phonebook is a [Rocket](https://rocket.rs) application, and if you are familiar with how to [configure Rocket applications](https://rocket.rs/v0.5-rc/guide/configuration/) you are free to configure it how you wish, including Rocket.toml and other methods.

For those who arenn't familiar with how to configure Rocket applications, run Yealink Phonebook with environment variables:

```bash
ROCKET_VCF_URL=https://my-cloud/dav/addressbooks/users/$USER/contacts?export # REQUIRED, specifies the location of the vCard file
ROCKET_PORT=80 # Specifies the port the application will run on, 8000 default
ROCKET_LOG_LEVEL=normal
ROCKET_ADDRESS=0.0.0.0 # Bind on all interfaces
```

## Running Yealink Phonebook

```bash
ROCKET_VCF_URL=http://my-server/contacts.vcf yealink-phonebook
```

## Docker Compose

```yml
version: "3"
services:
  phonebook:
    build: .
    ports:
      - 8000:80
    environment:
      - ROCKET_VCF_URL=http://url
      - ROCKET_PORT=80
      - ROCKET_LOG_LEVEL=normal
      - ROCKET_ADDRESS=0.0.0.0
    restart: unless-stopped
```
