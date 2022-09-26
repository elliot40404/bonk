<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/elliot40404/bonk">
    <img src="bonk.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Bonk</h3>

  <p align="center">
    The blazingly fast touch alternative written in rust. Made for the sole purpose to create files.
    <br />
    <a href="https://github.com/elliot40404/bonk"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/elliot40404/bonk">View Demo</a>
    ·
    <a href="https://github.com/elliot40404/bonk/issues">Report Bug</a>
    ·
    <a href="https://github.com/elliot40404/bonk/issues">Request Feature</a>
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
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

There are many ways to create a file via cli depending on the OS you are on but me currently being on windows, though some command are aliased they don't feel natural (`New-Item` instead of `touch`) and I miss using some of the unix commands that I am so used to. Thats how bonk came to be.

Here's why:

-   Works on all OS'
-   Can Also create directories recursively
-   Why not?

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

### Installation

_Below is an example of how you can instruct your audience on installing and setting up your app. This template doesn't rely on any external dependencies or services._

1. Clone the repo
    ```sh
    git clone https://github.com/elliot40404/bonk.git
    ```
1. Build and Install the rust binary
    ```sh
    cd bonk
    cargo install --path .
    ```

or 

```sh
cargo install bonky
```
Note: name of the executable is `bonk` though the crate is named bonky.

Download the binary from the releases page

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

Create a file

```bash
bonk main.rs
```

Create a directory

```bash
bonk src/
```

Create a nested file

```bash
bonk src/main.rs
```

Create nested directories

```bash
bonk src/bonky/
```

Create nested file inside nested directories

```bash
bonk src/bonky/mod.rs
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## Roadmap

-   [ ] Add timestamp modification functionality

See the [open issues](https://github.com/othneildrew/Best-README-Template/issues) for a full list of proposed features (and known issues).

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

[contributors-shield]: https://img.shields.io/github/contributors/elliot40404/bonk.svg?style=for-the-badge
[contributors-url]: https://github.com/elliot40404/bonk/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/elliot40404/bonk.svg?style=for-the-badge
[forks-url]: https://github.com/elliot40404/bonk/network/members
[stars-shield]: https://img.shields.io/github/stars/elliot40404/bonk.svg?style=for-the-badge
[stars-url]: https://github.com/elliot40404/bonk/stargazers
[issues-shield]: https://img.shields.io/github/issues/elliot40404/bonk.svg?style=for-the-badge
[issues-url]: https://github.com/elliot40404/bonk/issues
[license-shield]: https://img.shields.io/github/license/elliot40404/bonk.svg?style=for-the-badge
[license-url]: https://github.com/elliot40404/bonk/blob/master/LICENSE
[product-screenshot]: bonk.png