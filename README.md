# MYT Automation Server
MYT Automation server is a server that conects the microservices architecture to Power Automate, a low code tool for easy integration & automation

## Getting started
### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
  > :bulb: While this is listed within the requirements, you'll only need this if you choose to run your code locally. Docker takes care of Rust!
- [Docker Desktop](https://www.docker.com/get-started)

### Setup
1. Clone the project to your local machine using your favorite CLI & `git clone`
2. Copy the `.env.example`, rename it to `.env` and fill it in according to the [`.env` configuration section](#env-configuration).
3. Run the following command:
    ```bash
    $ docker-compose --profile container-dev up -d
    ```
    > 🚨 The first build might take a while. Rust takes a while to build

    > 📥 If you wish the run your code locally for any reason, you can use the following command: ``` $ docker-compose --profile local-dev up -d ```
# Miscallanious
## `.env` configuration
> :bulb: There's a `.env.example` file available to get you started!

|Key|Value explanation|Required?|Value example|
|---|---|---|---|
|RABBITMQ_PORT|The port used by RabbitMQ|✔|5672|
|WARP_PORT|The port used by WARP|✔|3030|
