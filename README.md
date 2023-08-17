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
	A RESTful authentication and authorization API written in Rust (specifically using axum) and web app using Leptos and TailwindCSS. All data is stored in Redis or PostgreSQL with tokio async support using diesel_async and deadpool. Targeting OAuth2 specifications.
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
* [![Axum][Axum.rs]][Axum-url]
* [![Leptos][Leptos.rs]][Leptos-url]
* [![TailwindCSS][Tailwind.css]][Tailwind-url]
* [![Diesel][Diesel.rs]][Diesel-url]
* [![PostgreSQL][PostgreSQL.psql]][PostgreSQL-url]
* [![Redis][Redis.redis]][Redis-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started
To get a local copy up and running follow these steps.

### Prerequisites

To run this application, you will need to have cargo and cargo-leptos installed, and Redis and PostgreSQL setup
* Cargo [installation docs](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* Cargo-leptos [installation docs](https://github.com/leptos-rs/cargo-leptos)
* Redis [download page](https://redis.io/download/)
* PostgreSQL [download page](https://www.postgresql.org/download/)

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

1. Setup your .env file with the database path and secrets
    ```sh
    echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
    echo REDIS_URL=redis://localhost:6379 > .env
    echo KEY_INTERVAL={Seconds} > .env
    echo AUTH_INTERVAL={Seconds} > .env
    ```

1. Install the diesel CLI and initialize diesel in the project
   ```sh
   # run this command in the server project root e.g. .../lockrs/server
   cd server
   cargo install diesel_cli
   diesel setup
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

By default, the server runs on port 9000, though this can be changed by changing the port number defined in the main function in server/main.rs.

_Example Auth Flow_
```sh
    # start up server
    cargo run

    curl -X POST http://127.0.0.1:9000/api/v1/auth \
        -H 'Authorization: Basic <Basic Auth Credentials>'

    # user recieves session token in response, e.g.
    # { "session_token": <Some nonce>, "expires_at": <UNIX timestamp> }

    curl -X POST http://127.0.0.1:9000/api/v1/session \
        -c ./lockrs.cookies -b ./lockrs.cookies \
        -H 'Authorization: Bearer <session_token value>'

    # jwt cookie is set, and session token has been consumed and is not longer expired.
```

For convenience, a few standard requests have been stored in server/curls. If you want to run them, check out the scripts to see what params are required, and chmod +x the server/curls/* directory if you need to run anything. 

### Running the web app on /frontend

To start the web application, first we must start the backend api using the steps above. This project requires the use of nightly rust, so make sure to run:

```sh
    # run in /path/to/lockrs/frontend
    rustup override set nightly
```

After you have the backend running, open a new terminal instance and run

```sh
    # run if not in frontend from last step already.
    # Go to frontend binary directory
    cd frontend

    cargo leptos watch
```

From this point, open up a browser and navigate to http://127.0.0.1:8000/signup and register a new user, /login to authenticate existing users, etc.

If you do plan on making any changes to styling, make sure to have a terminal running:

```sh
    # run in /path/to/lockrs/frontend
    npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

to enable hot rebuilding of tailwind as you develop!

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
  - [x] Switch from Yew to Leptos
  - [ ] Create pages for /user operations
  - [ ] Create pages for /client operations
  - [ ] Create pages for /redirect uri operations
- [ ] Backend
  - [x] Finish /api controllers
  - [ ] Finish /oauth2 controllers
  - [ ] Add scopes to authorization functionality
  - [ ] Add support of OIDC

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
[Leptos.rs]: https://img.shields.io/badge/Leptos-ef3939?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAAAIGNIUk0AAHomAACAhAAA+gAAAIDoAAB1MAAA6mAAADqYAAAXcJy6UTwAAAD5UExURQAAABYLNxgMNxcNNhcNNufh5es3Ou4zOAAAHxcNNhgNN+s5Pfi4utra2hgNOBoQOgAAJBcNNRcLNhUKNhYLMxcLNhgMNxcMNxcNNxgNNxcNNxcNNhcNNxgNNmVeev///0c+XxcNNxkOOMrH0fBpbJmUp+jm6u/m6ZyXqTsyVdjW3SAVPjguU7Ctu+zk6Pfv8MzK0zctUjgvU+vf4vF3euw5PfF4e7GuvPF3eew3O6+ruu7l6Ow4POvk5+zk57KuvPF2efF4erCsujkvU+zf4uvf4zcuUrazwPz09O3l6LGtu3hxip2YqkA4Wt7d4i0jSd7d40pBYv///+fxzp8AAAAidFJOUwAulNb21ZMtCJf9/ZUHv70HmJUvLZaT2NX4mJbAv/0vltcrchWcAAAAAWJLR0QfBQ0QvQAAAAd0SU1FB+cHDgApH1wkuKcAAAC5SURBVBjTTU/HFoJAEBsUFbCCFTvYFbtYsWLv/v/POLN4MIdskre7kwFAcC43z3u8PnAgiJKOKJX9gSDzIZ2hUq3VwxEMZMc3mi1kBSBK9412p9vro5BiEMdjMByZ5nhISQKSyJPpDDGeo0yBirywKDCXKPn/YMUCerLeULDdoXSzT+39xrJGhyNKlzPWPp0vV4PGcgBpqnW7O/UyVD2r64/ni/mcwJaR8+8PWUkUfvsWipqqagpH+gsu0CAuf9g+3gAAACV0RVh0ZGF0ZTpjcmVhdGUAMjAyMy0wNy0xNFQwMDo0MTozMSswMDowMFuu09YAAAAldEVYdGRhdGU6bW9kaWZ5ADIwMjMtMDctMTRUMDA6NDE6MzErMDA6MDAq82tqAAAAKHRFWHRkYXRlOnRpbWVzdGFtcAAyMDIzLTA3LTE0VDAwOjQxOjMxKzAwOjAwfeZKtQAAAABJRU5ErkJggg==
[Leptos-url]: https://leptos.dev/
[Tailwind.css]: https://img.shields.io/badge/Tailwind_CSS-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white
[Tailwind-url]: https://tailwindcss.com/
[Redis.redis]: https://img.shields.io/badge/Redis-FF0000?style=for-the-badge&logo=redis&logoColor=white
[Redis-url]: https://redis.io/
[PostgreSQL.psql]:  	https://img.shields.io/badge/PostgreSQL-FFFFFF?style=for-the-badge&logo=postgresql&logoColor=blue
[PostgreSQL-url]: https://www.postgresql.org/
