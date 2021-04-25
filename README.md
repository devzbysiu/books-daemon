<div align="center">

  <h1><code>books-daemon</code></h1>

  <h3>
    <strong>Listen for new books added under specified directory and send them via bluetooth to
    specified device.</strong>
  </h3>

  <p>
    <img src="https://github.com/devzbysiu/books-daemon/workflows/CI/badge.svg" alt="CI status
    badge" />
    <a href="https://codecov.io/gh/devzbysiu/books-daemon">
      <img src="https://img.shields.io/codecov/c/github/devzbysiu/books-daemon?style=for-the-badge&token=f2339b3de9e44be0a902458a669c1160" alt="Code coverage"/>
    </a>
    <a href="https://crates.io/crates/books-daemon">
      <img src="https://img.shields.io/crates/l/books-daemon?style=for-the-badge" alt="License"/>
    </a>
    <a href="https://docs.rs/books-daemon">
      <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge" alt="docs.rs docs" />
    </a>
  </p>

  <h3>
    <a href="#about">About</a>
    <span> | </span>
    <a href="#installation">Installation</a>
    <span> | </span>
    <a href="#configuration">Configuration</a>
    <span> | </span>
    <a href="#license">License</a>
    <span> | </span>
    <a href="#contribution">Contribution</a>
  </h3>

  <sub><h4>Built with 🦀</h4></sub>
</div>

# <p id="about">About</p>

The idea is simple. When you execute this program, it will go to the background and it will start
watching for the changes under specified (in settings file) directory.

When new file appears, daemon picks it up and sends it via bluetooth to specified device.

Under the hood, it uses `bt-obex` to send the file via bluetooth, so it's required to make this
software working.

See [configuration](#configuration) section to see what can be configured.

# <p id="installation">Installation</p>

## --- TODO ---

# <p id="configuration">Configuration</p>

Example configuration is shown below. `books-daemon` will look for settings file under
`~/.config/books-daemon.toml`.

```toml
interval = 2                          # how often check the content of the books_dir
books_dir = "/home/zbychu/books"      # where should it listen for new books
device_mac = "64:A2:F9:E9:AE:C3"      # device MAC number to which new books should be send
stdout_file = "/tmp/books_daemon.out" # stdout logs
stderr_file = "/tmp/books_daemon.err" # stderr logs
```

# <p id="license">License</p>

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# <p id="contribution">Contribution</p>


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
