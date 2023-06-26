<a name="readme-top"></a>


[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/quasiuslikecautious/lockrs">
    <img src="quasius.dev.icon.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">lockrs<h3>

  <p align="center">
	A RESTful authentication and authorization API written in Rust (specifically using axum) and web app using Yew and Sylist. All data is stored in Redis or PostgreSQL with tokio async support using diesel_async and deadpool. Targeting OAuth2 specifications.
	<br />
	<a href="https://github.com/quasiuslikecautious/lockrs">
	  <strong>Explore the docs</strong>
	</a>
	<br />
	<br />
	<a href="https://github.com/quasiuslikecautious/lockrs">View Demo</a>
	.
	<a href="https://github.com/quasiuslikecautious/lockrs/issues">Report Bug</a>
	.
	<a href="https://github.com/quasiuslikecautious/lockrs/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

[![lockrs Screen Shot][product-screenshot]](https://github.com/quasiuslikecautious/lockrs)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Built With

* [![Rust][Rust.rs]][Rust-url]
* [![Diesel][Diesel.rs]][Diesel-url]
* [![Axum][Axum.rs]][Axum-url]
* [![Yew][yew.rs]][Yew-url]
* [![Redis][Redis.redis]][Redis-url]
* [![PostgreSQL][PostgreSQL.psql]][PostgreSQL-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started
To get a local copy up and running follow these steps.

### Prerequisites

To run this api, you will need to have cargo installed, and Redis and PostgreSQL setup
* Cargo [installation docs]("https://doc.rust-lang.org/cargo/getting-started/installation.html")
* Trunk [installation docs](https://trunkrs.dev/#install)
* Redis [download page](https://redis.io/download/)
* PostgreSQL [download page]("https://www.postgresql.org/download/")

After installing postgres, make sure you setup a database to be used with the api, e.g.

```sql
CREATE DATABASE lockrs;
```

<br />
<strong>Side Note:</strong>

I also highly recommend [cargo-watch]("https://crates.io/crates/cargo-watch") for allowing live reloads on the server as changes are saved. If you do go this route, don't forget to ignore the log folder or cargo will just keep restarting the server!
<br />
<br />

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/quasiuslikecautious/lockrs.git
   ```
1. Install cargo crates
   ```sh
   cargo build
   ```
1. Install the diesel CLI and initialize diesel in the project
   ```sh
   # run this command in the project root e.g. .../lockrs/
   cargo install diesel_cli
   diesel setup
   ```

1. Setup your .env file with the database path and secrets
    ```sh
    echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
    echo REDIS_URL=redis://localhost:6379 > .env
    echo KEY_INTERVAL={Seconds} > .env
    echo AUTH_INTERVAL={Seconds} > .env
    ```

1. Initialize your database with the tables this project will use
    ``` sh
    diesel migration run
    ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

### Running the API on /server
To start the API, simply run 

```sh
cd server # go to server binary directory
cargo run # default run command
# OR
cargo watch -x run -i log # if you have cargo-watch installed and want live reloads
```

in the project's root, and the server will start up. 

By default, the server runs on port 8081, though this can be changed by changing the port number defined in the main function in server/main.rs.

_Example Auth Flow_
```sh
    # start up server
    cargo run

    curl -X POST http://127.0.0.1:8081/api/v1/auth \
        -H 'Authorization: Basic <Basic Auth Credentials>'

    # user recieves session token in response, e.g.
    # { "session_token": <Some nonce>, "expires_at": <UNIX timestamp> }

    curl -X POST http://127.0.0.1:8081/api/v1/session \
        -c ./lockrs.cookies -b ./lockrs.cookies \
        -H 'Authorization: Bearer <session_token value>'

    # jwt cookie is set, and session token has been consumed and is not longer expired.
```

For convenience, a few standard requests have been stored in server/curls. If you want to run them, check out the scripts to see what params are required, and chmod +x the server/curls/* directory if you need to run anything. 

### Running the web app on /frontend

To start the web application, first we must start the backend api using the steps above. By default After you have the backend running, open a new terminal instance and run

```sh
    cd frontend # go to frontend binary directory
    trunk serve --proxy-backend=http://127.0.0.1:8081/api/ # create proxy and point it at running backend instance
```

From this point, open up a browser and navigate to http://127.0.0.1:8080/signup and register a new user, /login to authenticate existing users, etc.

_For more examples, please refer to the [Documentation](https://example.com) TODO will add link to API docs here_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Documentation
  - [x] Add a README
  - [ ] Create API documentation
  - [ ] Add docs to code
- [ ] Testing
  - [ ] Add unit tests
  - [ ] Add integration tests
  - [ ] Add a few end to end tests
- [ ] Frontend
  - [ ] Create pages for /user operations
  - [ ] Create pages for /client operations
  - [ ] Create pages for /redirect uri operations
- [ ] Backend
  - [ ] Add scopes to authorization functionality
  - [ ] Add support of OIDC
  - [ ] Finish operation implementations on redirect_uris

See the [open issues](https://github.com/quasiuslikecautious/lockrs/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Your Name - [@zquasius](https://twitter.com/zquasius) - zach@quasius.dev

Project Link: [https://github.com/quasiuslikecautious/lockrs](https://github.com/quasiuslikecautious/lockrs)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/quasiuslikecautious/lockrs.svg?style=for-the-badge
[contributors-url]: https://github.com/quasiuslikecautious/lockrs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/quasiuslikecautious/lockrs.svg?style=for-the-badge
[forks-url]: https://github.com/quasiuslikecautious/lockrs/network/members
[stars-shield]: https://img.shields.io/github/stars/quasiuslikecautious/lockrs.svg?style=for-the-badge
[stars-url]: https://github.com/quasiuslikecautious/lockrs/stargazers
[issues-shield]: https://img.shields.io/github/issues/quasiuslikecautious/lockrs.svg?style=for-the-badge
[issues-url]: https://github.com/quasiuslikecautious/lockrs/issues
[license-shield]: https://img.shields.io/github/license/quasiuslikecautious/lockrs.svg?style=for-the-badge
[license-url]: https://github.com/quasiuslikecautious/lockrs/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/zach-quasius/
[product-screenshot]: screenshot.png
[Rust.rs]:  	https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Diesel.rs]: https://img.shields.io/badge/diesel-535379?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAASCAIAAADUsmlHAAAACXBIWXMAAAs2AAALNgG/CNh6AAAAEXRFWHRUaXRsZQBQREYgQ3JlYXRvckFevCgAAAATdEVYdEF1dGhvcgBQREYgVG9vbHMgQUcbz3cwAAAALXpUWHREZXNjcmlwdGlvbgAACJnLKCkpsNLXLy8v1ytISdMtyc/PKdZLzs8FAG6fCPGXryy4AAADh0lEQVQYGVXBfWiUdQAH8J+103RzL+l5d8+z293z3Nvz3HO3tSyFKIqFNRJfIDYotWINQlIC6UU0hNG0crc57zI3nYZGNbEFIZtiRiBEWVGjbeosHOJ0OVs3d892z/P8Xr755+3zITAh4FgwQTk4AE5ZngGZ/bu31j5yKHN8BoCwYCLLOWCiABFwTMDmgkKAweYMnLbufvtzKf6mf3XnUjmTbocAwGcBcIYCJCf+Y8CMEOCcUyaA9o/3nNaNL/3ukz2pLZteaSur2J9qAYVjCoZ5CBhMKw/OQalDeerTjt7lqggEEY19EPF/09vb3NjQGnB3pI+CwoRAAQIHYIAAAw5m9p5Y4YOWRCx5L14DQ2+R3OcH+pue27zPV3n0UDccGwUIZw4DBNB64MNTSgjRqrwujwfjE5VqTo3ACOxU9FMXBjbVrdkpeSf/HUMBAlAIeqC9pU/WWMyYjcuTfumrB4t+LF1iawqM1VBCaY+0Q9OPyOrotV9QgOSBrk/aToYjiGlQY3+Ue8+UV4xXBRDTEI2waHgmXI1kEIbSURH4rv8sCpAvjn10QqpylDhqkrci6hlCpkIKtBCt9iOWuO4ODFZJCCsIJ9s8UjKkHunpHTjXB+TAQFaWL7lUoiJRi2j04uKiYVVF/FErERGKLx82zi8uG5XlfNjn1OjHPVXhpPZG85bNr23IHO5iAGnasLablF5+eNlkpNLxqxOSkjOSUCIiYQz65IvF5VldQ21sZHnZS4Rse/c9UPt039fP1L84+vdNMvPP7ZfrVu15aOGQxwtNR1K3FfWuFu0vcZ1zkTte2dZjN92+zxYW1wf8s3MOGECR6nzHngOBoOZ0du3zT6bJop+XleRDHkTD3xaRs6XFSFZzLTghVx4krnUr5FvXrs8xNgf8cOGnrdtLd+14izBBITCZnd347BOHyQNDqhdKxErUQHsMhjFaVtztcq2TpZGxGxZgA6C5fU2NY9/X7Vr/OHEYtQETcO5k1z+1ci8puuL2Q4tD1X8PBjoWLGqQfSNjV+7Z1AKo7eQo/rp6Y1vz0z2daQIBcBvMBsP41Exj/ZoUIYNSyYhvaRchDf7gn8NDnFNwG44NBgs0L/KCA3BIDpyCA9xidArirpV9NVH7vsuXWuBt8FQO/3aJAjYghAATNuYhYADjsCgc3JeDPT19e/vGF15fVXv56q8mKO7jAAUoGOb5HydLvO6P1HglAAAAAElFTkSuQmCC
[Diesel-url]: https://diesel.rs/
[Axum.rs]: https://img.shields.io/badge/axum-000000?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAD8AAAA4AQMAAABwlLIkAAAABlBMVEX///8AAABVwtN+AAAACXBIWXMAAA7EAAAOxAGVKw4bAAAA4ElEQVQYlVXRPYrDMBAFYAXDupQPsGD2HgFdacsUxpNULn2lQC4SxxdwOhcPTaSnH2JVH/phNG+MelHd1RuFC5AA36tucUcD3oSFLMQMWYkJ8iAekCkhnBILxBJvSE9sGBN2DC5hGITA+UKs/nzRmfj9Jxbt/oiXM6c9Avba8GO+bW53Pr/2W0M83d6mWoJcS5Brqc/AGBEWRpNWAo8y6uXyfCsIJeyhqP9p7zdGZ83pKd9fffmuA7uofcVO59x7CWF0x3xqYjXD9Sve6Zj8VGZRp9NDOK86QV9mGocLQogPHV1XySZN2nYAAAAASUVORK5CYII=
[Axum-url]: https://github.com/tokio-rs/axum
[Yew.rs]: https://img.shields.io/badge/yew-FFFFFF?style=for-the-badge&logo=data:image/png;base64,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
[Yew-url]: https://yew.rs
[Redis.redis]: https://img.shields.io/badge/Redis-FF0000?style=for-the-badge&logo=redis&logoColor=white
[Redis-url]: https://redis.io/
[PostgreSQL.psql]:  	https://img.shields.io/badge/PostgreSQL-FFFFFF?style=for-the-badge&logo=postgresql&logoColor=blue
[PostgreSQL-url]: https://www.postgresql.org/
