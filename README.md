# MYT Automation Server
MYT Automation server is a server that connects the microservices architecture to Power Automate, a low code tool for easy integration & automation.

- Power automate flow import files: https://drive.google.com/file/d/1QAOZqiB59EFrZr-vxGIVhMh-kxBMyWGN/view?usp=sharing

## Getting started
### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
  > :bulb: While this is listed within the requirements, you'll only need this if you choose to run your code locally. Docker takes care of Rust!
- [Docker Desktop](https://www.docker.com/get-started)

## Setup
> ‼ When within staging & production, the image will need to be rebuild when code changes happen. 
> For this, add the `--build` argument behind the given `docker-compose` commands

### Development
1. Clone the project to your local machine using your favorite CLI & `git clone`
2. Copy the `.env.example`, rename it to `.env` and fill it in according to the [`.env` configuration section](#env-configuration).
3. Run the actual project
- If you want to develop **within** the container, you'll need to use the following command:
```bash
$ docker-compose --profile dev-container up
```
> 🚨 The first build might take a while. Rust takes a while to build.

- If you want to run your build locally, you'll need to use the following command:
```bash
$ docker-compose --profile dev-local up
```
4. When you want to get the containers offline, enter the following command:
```bash
# If you were developing locally
$ docker-compose --profile dev-local down

# If you were developing within containers
$ docker-compose --profile dev-container down
```
### Staging
1. Clone the project to your local machine using your favorite CLI & `git clone`
2. Copy the `.env.example`, rename it to `.env` and fill it in according to the [`.env` configuration section](#env-configuration).
3. Build the project
```bash
$ docker-compose --profile staging up --build -d
```
> ⁉ This staging build will simulate a production environment, but will leave all debug ports & tools available.
4. When you want to get your app offline, enter the following command:
```bash
$ docker-compose --profile staging down
```

### Production
1. Clone the project to your local machine using your favorite CLI & `git clone`
2. Copy the `.env.example`, rename it to `.env` and fill it in according to the [`.env` configuration section](#env-configuration).
3. Build the project
```bash
$ docker-compose --profile prod up --build -d
```
4. When you want to get your app offline, enter the following command:
```bash
$ docker-compose --profile prod down
```

#### Adding other services
Because [Gateway](https://git.ti.howest.be/TI/2021-2022/s5/trending-topics/projects/hybrid-work1/gateway), [Automation API](https://git.ti.howest.be/TI/2021-2022/s5/trending-topics/projects/hybrid-work1/automateapi) and [Back-End](https://git.ti.howest.be/TI/2021-2022/s5/trending-topics/projects/hybrid-work1/back-end) share the same custom network, you can build these and get the production server up & running.

- For [Back-End](https://git.ti.howest.be/TI/2021-2022/s5/trending-topics/projects/hybrid-work1/back-end), clone the project, navigate to the root of the project and perform the following command:
  ```bash
  $ docker-compose up -d nestjs_dev
  $ docker-compose up -d mongodb_dev
  ```
  > :warning: Currently, the GraphQL implementation in the production container seems to be broken, so we suggest using the development version for now.

- For [Gateway](https://git.ti.howest.be/TI/2021-2022/s5/trending-topics/projects/hybrid-work1/gateway), clone the project, navigate to the root of the project and perform the following command:
  ```bash
  $ docker-compose up tsed_prod -d
  ```

# Miscellaneous
## `.env` configuration
> :bulb: There's a `.env.example` file available to get you started!

| Key| Value explanation|Value example|
|---|---|---|
|COMPOSE_PROJECT_NAME|Name of the compose stack|MYT Automate Server|
|AUTOMATE_URL| The url used **outside** the container network to communicate with Power Automate|https://prod-39.westeurope.logic.azure.com:443/workflows/|
||||
|RABBITMQ_ENDPOINT|The address used **within** the app to reach Rabbit MQ|localhost|
|RABBITMQ_PORT|The port used **within** the app to communicate with Rabbit MQ|5672|
|RABBITMQ_QUEUE|The name of the queue **within** the app to listen to from Rabbit MQ|bookings|
|RABBITMQ_USERNAME|The username **within** the app to listen to from Rabbit MQ|guest|
|RABBITMQ_PASSWORD|The password **within** the app to listen to from Rabbit MQ|guest|
||||
|DOCKER_RABBITMQ_ENDPOINT|The address used to reach RabbitMQ **inside** the container|localhost|
|DOCKER_RABBITMQ_PORT|The port used to communicate with Rabbit MQ **inside** the container|5672|
|DOCKER_RABBITMQ_EXTERNAL_PORT|The port used **outside** the container network to communicate with Rabbit MQ|5672|
|DOCKER_RABBITMQ_EXTERNAL_MANAGEMENT_PORT|The port used **outside** the container network to communicate with Rabbit MQ's Manager interface| 15672 |
