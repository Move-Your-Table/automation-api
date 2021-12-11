# MYT Automation Server
MYT Automation server is a server that conects the microservices architecture to Power Automate, a low code tool for easy integration & automation

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
   $ docker-compose --profile staging up
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
   $ docker-compose --profile prod up
   ```
4. When you want to get your app offline, enter the following command:
   ```bash
   $ docker-compose --profile prod down
   ```
# Miscellaneous
## `.env` configuration
> :bulb: There's a `.env.example` file available to get you started!

|Key|Value explanation|Required for Development?|... Staging?|... Production?|Value example|
|---|---|---|---|---|---|
|COMPOSE_PROJECT_NAME|Name of the compose stack|✖|✖|✖|MYT Automate Server|
|||||||
|WARP_PORT|The port used to reach the application from external networks|✔|✔|✔|3030|
|||||||
|RABBITMQ_ENDPOINT|The address used **within** the container network to reach Rabbit MQ|✔* </br> *\*If the code's running locally*|✖|✖|localhost|
|RABBITMQ_PORT|The port used **within** the container network to communicate with Rabbit MQ|✔* </br> *\*If the code's running locally*|✖|✖|5672|
|RABBITMQ_EXTERNAL_PORT|The port used **outside** the container network to communicate with Rabbit MQ|✔|✔|✖|5672|
|RABBITMQ_EXTERNAL_MANAGEMENT_PORT|The port used **outside** the container network to communicate with Rabbit MQ|✔|✔|✖|15672|


