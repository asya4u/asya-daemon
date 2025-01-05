<a id="readme-top"></a>


<!-- PROJECT LOGO //TODO Add lodo -->
<br />
<div align="center">
  <a href="https://github.com/asya4u/asya-daemon">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  # Who is Asya?

  <p align="center">
    Your personal digital assistant for PC use. Asya helps you to manage your system,
    </br>
    get information about the status of your PC, update and troubleshoot problems.
    <br />
    <a href="https://github.com/DmitryHudrich/asya-daemon/wiki"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <!-- <a href="https://github.com/asya4u/asya-daemon">View Demo</a>  //TODO -->
  </p>

  ![Badge Pull Requests]
  ![Badge Issues]
  ![Badge Hi Mom]
  ![Badge Language]
  ![Badge License]
  [![channel icon](https://patrolavia.github.io/telegram-badge/follow.png)](https://t.me/asya_sillyblog)
  
  **[<kbd> <br> Report Bug <br> </kbd>][Bug]**
  **[<kbd> <br> Request Feature <br> </kbd>][Feature]**

  ![](https://count.getloli.com/get/@asya4u.github.readme)

  ---
</div>

# 🐳 Getting started

## Unix

### Install dependencies:
  * For all distros use [rustup](https://rustup.rs) as recommended way to use cargo package manager.

  * Debian-based
      ```sh
      sudo apt update
      sudo apt install -y luajit pkg-config libssl-dev
      ```
      
  * Fedora 
      ```sh
      sudo dnf install -y luajit pkgconf-pkg-config openssl-devel
      ```
  * Arch-based
      ```sh
      sudo pacman -Syu
      sudo pacman -S --noconfirm luajit pkgconf openssl
      ```
  * macOS:
      ```sh
      brew update
      brew install luajit pkg-config openssl
      ```
### Build
```sh
cargo install --git https://github.com/asya4u/asya-daemon
```
### NixOS

  If you use package manager nix you can run project using nix-shell.

  - For build flake run
  ```sh
  nix build
  ```

  - For run environment
  ```sh
  nix develop
  ```


<!-- USAGE EXAMPLES -->
## 🐳 Usage

All that remains is to wait :)

<!-- CONTRIBUTING -->
## 🎁 Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Top contributors:

<a href="https://github.com/asya4u/asya-daemon/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=asya4u/asya-daemon" alt="contrib.rocks image" />
</a>



<!-- LICENSE -->
## 🧑‍⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

```
⣿⡇⣿⣿⣿⠛⠁⣴⣿⡿⠿⠧⠹⠿⠘⣿⣿⣿⡇⢸⡻⣿⣿⣿⣿⣿⣿⣿ █▀▄ █ █▀▄   █▄█ █▀█ █░█   █▄▀ █▄░█ █▀█ █░█░█ ▀█              
⢹⡇⣿⣿⣿⠄⣞⣯⣷⣾⣿⣿⣧⡹⡆⡀⠉⢹⡌⠐⢿⣿⣿⣿⡞⣿⣿⣿ █▄▀ █ █▄▀   ░█░ █▄█ █▄█   █░█ █░▀█ █▄█ ▀▄▀▄▀ ░▄              
⣾⡇⣿⣿⡇⣾⣿⣿⣿⣿⣿⣿⣿⣿⣄⢻⣦⡀⠁⢸⡌⠻⣿⣿⣿⡽⣿⣿                                                                 
⡇⣿⠹⣿⡇⡟⠛⣉⠁⠉⠉⠻⡿⣿⣿⣿⣿⣿⣦⣄⡉⠂⠈⠙⢿⣿⣝⣿                                                                 
⠤⢿⡄⠹⣧⣷⣸⡇⠄⠄⠲⢰⣌⣾⣿⣿⣿⣿⣿⣿⣶⣤⣤⡀⠄⠈⠻⢮                                                                 
⠄⢸⣧⠄⢘⢻⣿⡇⢀⣀⠄⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⡀⠄⢀                                                                 
⠄⠈⣿⡆⢸⣿⣿⣿⣬⣭⣴⣿⣿⣿⣿⣿⣿⣿⣯⠝⠛⠛⠙⢿⡿⠃⠄⢸ █▀▀ █▀█ █▀▀ █▀▀ █▀▀ █▀▀   █▀█ █▀█ █▀█ ▀█▀ █▀▀ █▀▀ ▀█▀ █▀     
⠄⠄⢿⣿⡀⣿⣿⣿⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣿⣿⡾⠁⢠⡇⢀ █▄▄ █▄█ █▀░ █▀░ ██▄ ██▄   █▀▀ █▀▄ █▄█ ░█░ ██▄ █▄▄ ░█░ ▄█     
⠄⠄⢸⣿⡇⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣏⣫⣻⡟⢀⠄⣿⣷⣾                                                                 
⠄⠄⢸⣿⡇⠄⠈⠙⠿⣿⣿⣿⣮⣿⣿⣿⣿⣿⣿⣿⣿⡿⢠⠊⢀⡇⣿⣿ █▀▀ █▀█ █▀█ █▀▄▀█   ▄▀█ █░░ ▀█ █░█ █▀▀ █ █▀▄▀█ █▀▀ █▀█ ▀ █▀ ░
⠒⠤⠄⣿⡇⢀⡲⠄⠄⠈⠙⠻⢿⣿⣿⠿⠿⠟⠛⠋⠁⣰⠇⠄⢸⣿⣿⣿ █▀░ █▀▄ █▄█ █░▀░█   █▀█ █▄▄ █▄ █▀█ ██▄ █ █░▀░█ ██▄ █▀▄ ░ ▄█ ▄
      
                                                                         
```


<!-- MARKDOWN LINKS & IMAGES -->
[Badge Issues]: https://img.shields.io/github/issues/DmitryHudrich/asya-daemon
[Badge Pull Requests]: https://img.shields.io/github/issues-pr/DmitryHudrich/asya-daemon
[Badge Language]: https://img.shields.io/github/languages/top/DmitryHudrich/asya-daemon
[Badge Lines]: https://img.shields.io/tokei/lines/github/hyprwm/DmitryHudrich/asya-daemon
[Badge Hi Mom]: https://img.shields.io/badge/Hi-mom!-ff69b4
[Badge Language]: https://img.shields.io/github/languages/top/DmitryHudrich/asya-daemon
[Badge License]: https://img.shields.io/github/license/DmitryHudrich/asya-daemon

[Feature]: https://github.com/DmitryHudrich/asya-daemon/issues/new?labels=bug&template=bug-report---.md
[Bug]: https://github.com/DmitryHudrich/asya-daemon/issues/new?labels=bug&template=bug-report---.md
