# Developing

- [Developing](#developing)
	- [Conventions](#conventions)
	- [Using Nix](#using-nix)
	- [Not using Nix](#not-using-nix)
	- [I Know What I'm Doing, Thank You Very Much](#i-know-what-im-doing-thank-you-very-much)
	- [Extra spice](#extra-spice)
		- [JSON IDE support](#json-ide-support)
		- [Tests](#tests)

These instructions have lots of information for new programmers. If you know what you're doing, read [this section](#i-know-what-im-doing-thank-you-very-much).

## Conventions

* Text that looks `like this` or 
  ```
	like this
	```
	is a command that should be run in your system's terminal.
* Text in "quotes" should be typed without the quotes.
* "Unix-like system" includes Linux and MacOS.

## Using Nix

Nix is only available for Unix-like systems.

1. `curl -L https://nixos.org/nix/install | sh` - Install Nix, if you haven't already.
2. `nix-env -i git vscode jq` - Install Git, VSCode, and JQ, if you haven't already.
3. Move your terminal to your projects directory, like "~/Code"
4. Clone the repository
   * If you're using SSH...
     1. Upload your public key to GitHub
     2. `git clone git@github.com:chromezoneeths/ethsbell-rewrite.git`
   * Otherwise...
     1. `git clone https://github.com/chromezoneeths/ethsbell-rewrite.git`
5. `cd ethsbell-rewrite` - Move your terminal to the newly cloned repository.
6. `./.vscode/extensions.sh` - Install our recommended extensions.
7. `code .` - Open VSCode in the newly closed repository.
8. If VSCode asks you to trust our repository, please do.
9. If you're running newer versions of OpenSuSE (or if `test -f "/etc/services"` fails and `test -f "/usr/etc/services"` succeeds) run `sudo ln -s /usr/etc/services /etc/services`.
10. Press Ctrl+Shift+P and type "nix sel", enter, "shell", enter.
11. If all goes well, there should be a prompt to reload in the bottom-left. Accept it and reload, then wait a minute or so.
12. If everything goes to plan, you should have everything you need to explore our repository.

## Not using Nix

1. Install Git, VSCode, and maybe jq...
   * If you're on Windows...
     * If you have winget...
       * `winget install -e --id Microsoft.VisualStudioCode`
       * `winget install -e --id Git.Git`
     * If you have chocolatey...
       * `choco install vscode git`
     * If you have neither...
       * https://github.com/git-for-windows/git/releases
       * https://code.visualstudio.com/
   * If you're on Linux...
     * This is long because Microsoft insists on using a proprietary license for some parts of VSCode, so most distributions don't package it.
     * (To prevent issues with pasting, run `sudo echo` first to start the authentication grace period, otherwise your shell might allow the paste to spill into sudo's password prompt.)
     * On a Deb distro (Debian, Ubuntu)...
       * ```bash
         # Import MS key
         wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages microsoft.gpg
         sudo install -o root -g root -m 644 packages.microsoft.gpg /etc/apt/trusted.gpg.d/
         sudo sh -c 'echo "deb [arch=amd64,arm64,armhf signed-by=/etc/apt/trusted.gpg.d/packages microsoft.gpg] https://packages.microsoft.com/repos/code stable main" > /etc/apt/sources.list d/vscode.list'
         rm -f packages.microsoft.gpg
         # Install something all distros really should ship with
         sudo apt install apt-transport-https
         # Install
         sudo apt update
         sudo apt install code git jq
         ```
     * On an RPM distro (Fedora, SuSE)...
       * ```bash
         # Import MS key
         sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
         # Add repo
         sudo sh -c 'echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" > /etc/yum.repos.d/vscode.repo'
         # Determine which package manager we have
         if which dnf; then
           export PM=dnf
         elif which zypper; then
           export PM=zypper
         fi
         # Install
         $PM install code git jq
         ```
   * If you're on MacOS...
     * If you have homebrew: `brew install --cask visual-studio-code && brew install git jq`
     * If you don't:
       * https://code.visualstudio.com/
       * XCode includes git, install that.
2. Install Rust
   * On 64-bit Windows, https://win.rustup.rs/x86_64
   * On 32-bit Windows, https://win.rustup.rs/i686
   * On Unix-like systems, `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Move your terminal to your projects directory, like "~/Code"
4. Clone the repository
   * If you're using SSH...
     1. Upload your public key to GitHub
     2. `git clone git@github.com:chromezoneeths/ethsbell-rewrite.git`
   * Otherwise...
     1. `git clone https://github.com/chromezoneeths/ethsbell-rewrite.git`
5. `cd ethsbell-rewrite` - Move your terminal to the newly cloned repository.
6. If you're on a Unix-like system and installed jq, `./.vscode/extensions.sh`
7. `code .` - Open VSCode in the repository.
8. If VSCode asks you to trust our repository, please do.
9. If step 6 didn't apply to you, press Ctrl+P and type "ext ins", press enter, type "@recommended:workspace" in the left sidebar, and install all of the extensions shown there, except those mentioning Nix, since you're not using that.
10. If everything goes to plan, you should have everything you need to explore our repository.

## I Know What I'm Doing, Thank You Very Much

1. Install VSCode, Git, and (if you're on a Unix-like system) jq.
2. Clone our repository.
3. Run extensions.sh under .vscode if you installed jq.
4. Otherwise, install all of the workspace recommended extensions.
5. If using Nix, load the profile in shell.nix.
6. Otherwise, install Rust.

## Extra spice

### JSON IDE support

When specifying a schedule type in `def.d`, you can add a property `"$schema": "../schema/Map_of_ScheduleType.json"` and run `cargo run --features ws --bin bell_mkschema` to generate files that will tell your IDE exactly what should and shouldn't be in there.

Technically you could name a schedule `$schema`, which would break this functionality, so please don't do that.

### Tests

If you want to make sure your code will work correctly, run `cargo test --features ws` before committing. Code that fails these tests will be rejected by our build automation. If you make a change that breaks an existing test and you don't think it's a bug in the test, that change requires a major version bump, since it breaks compatibility.