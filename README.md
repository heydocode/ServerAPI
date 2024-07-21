# ServerAPI

ServerAPI is a versatile software that allows you to turn any machine (Windows, Linux, and macOS) into a server. It sends machine statistics via email (configurable via setup wizard), display and register logs.

## IMPORTANT
ServerAPI is in "active" (now is not) developpement and is not released yet!

## Tasks

- Communication between threads
- When an email login attempt error occurs, launch the setup wizard to change infos (in parallel of other tasks to avoid problems if the owner isn't at the server)
- Do a better error handling system
- To do a list of languages in config/languages/..*.txt*
- Set device's language as a ServerAPI's local language
- Add tests
- Manage process calling and support of third party programs launching as a child process (like a minecraft software written in java, papermc.jar, or a mcbe software written in php, pocketmine-mp.phar)
- Add a safe connection to the ServerAPI terminal
- Add different permissions inside of ServerAPI (to avoid security failures for example by launching a virus as a child process)

## Features

- Easy installation and setup
- Sends machine statistics via email
- Supports running a Minecraft server with monitoring

## Requirements

- Access to Wi-Fi
- A compatible device (iOS and Android devices are not supported)

## Installation

1. Download and install the executable file.
2. Launch the software.
3. Follow the setup wizard.
4. Enjoy using ServerAPI!


## Configuration

ServerAPI includes a setup wizard that guides the user through the necessary configuration steps. The user simply needs to answer the wizard's questions and confirm the provided information.

## Support

For support, please create an "issue" on the GitHub page of the software.

## Contributing

We welcome contributions! If you would like to contribute, please fork the repository and create a pull request. We are open to any suggestions for improvement.

## License

This project is open-source and available under the MIT license. You are free to use, modify, and distribute the code as needed.

## Acknowledgments

Special thanks to the Rust programming language community and the creators of the "lettre" Rust crate for making this project possible.
