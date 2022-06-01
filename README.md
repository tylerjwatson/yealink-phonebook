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

## Running on Yealink Phones
1. Access the phone's web interface and log in using your administrator credentials
2. Click **Directory** in the top menu
   ![image](https://user-images.githubusercontent.com/5001408/171384609-2cc58c2b-a905-4abc-8051-2c664513309b.png)
3. Click **Remote Phonebook** on the left-hand submenu
4. Fill out the URL and display name of your running yealink-phonebook server
   ![image](https://user-images.githubusercontent.com/5001408/171385076-3cafa4d4-d2f9-4a5e-be8f-5b4492b17ba8.png)
5. The contact list served by yealink-phonebook should now be visible on the device.

