# Conveyancing CLI

A simple CLI application for conveyancing as a stop-gap measure until the full application has been developed.

## Installation

Open up the 'terminal' application and paste the following into it:

```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

This will install the 'brew' MacOS package manager. Brew is a simple and safe way to install and manage programs from the command line.

Next, install Iterm2 using the following command:

```shell
brew install --cask iterm2
```

From now on use Iterm2 as your terminal application!
Iterm2 is a great interactive terminal that can be customized to look and feel the way you want it to.

Next we will need to install Rust:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Just follow the defualt install instructions. When the Conveyancing CLI is finished we will be able to install it using the following:

```
cargo install conveyance
```

### Optional

Then set the default shell to ZSH as it is faster than the default BASH shell.

Install ZSH using:

```shell
brew install zsh
```

```shell
chsh -s /bin/zsh
```

And follow the instructions [here](https://iterm2colorschemes.com/) on how to set a colour scheme!

## Usage

To use the program you will pass a series of files, flags and inputs to the application via the Iterm2 terminal.

If you are confused about how to use the program at any point, type in the following:

```shell
conveyance -h
```

**OR**

```shell
conveyance --help
```

**TODO: Write documentation!**
