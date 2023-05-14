# Invoice Generator

This invoice generator is a binary application that calculates power consumption based on inverter yield and export data. The front end was made using the GTK-RS library.


## Installation

1. [Download Rustup and install Rust](https://www.rust-lang.org/tools/install) to compile the application.

2. Download the Windows [MSYS2 installer](https://www.msys2.org/). An MSYS2 shell should automatically open up once MYSYS2 has finished installing. Do not close it!

3. Install GTK4 and other necessary packages.

```bash
pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gettext mingw-w64-x86_64-libxml2 mingw-w64-x86_64-librsvg mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc mingw-w64-x86_64-libadwaita
```

> If you do close the MSYS2 shell after installing, you can find it by searching for `MSYS2 MinGW 64-bit` in the start menu.

4. Update your `Path` environment variable.

    1. Go to Settings -> Search and open `Advanced system settings` -> Click on `Environment variables`

    2. Select `Path` -> Click on `Edit`

    3. Add the following paths:

    ```bash
    C:\msys64\mingw64\include
    C:\msys64\mingw64\bin
    C:\msys64\mingw64\lib
    ```

5. Set up the GNU toolchain for Rust.

    - Install the GNU toolchain:

    ```bash
    rustup toolchain install stable-gnu
    ```

    - Set it as the default toolchain (replacing `stable-msvc`):

    ```bash
    rustup default stable-gnu
    ```

## Compilation

- To build the app, make sure you are in the project directory root directory and use Cargo:

    ```bash
    cd <project-directory>
    cargo build --release
    ```

> To compile the app in debug mode, omit the `--release` flag. To compile **and** run the app in debug mode execute `cargo run`.

## Usage

### Preparation

- Combine all the required yield and revenue summaries into a single XLS workbook with the same shared headers:

### Interacting with the App

- Go to `<project-directory>/target/release>` and run the executable.

#### Main window

- Open the respective combined summary workbook by clicking on Open Records.
- Open a new window where hours can be entered by clicking on Enter Hours or open an existing CSV file containing the respective hour values and headings. This must match the layout seen in the Hours window.
- The text entries next to or below each button will populate with the path to the respective files.
- Click the Generate button to generate an invoice from the given data, selecting a location for the saved file from the dialog that pops up.
- The Generate button will not become sensitive until both entries are populated with paths to files with appropriate file extensions.

#### Hours window

- Enter peak and off-peak hours in the table, providing start and end values for each day.
- Off-peak start and off-peak end hours are required for each day.
- Hour values are relative to a 24-hour clock; thus a value of 0 corresponds to 00h00 (12 a.m.) and value of 23 corresponds to 23h00 (11 p.m.), etc.
- Click the Save button to save an hours file, selecting its location from the dialog that pops up.
- The Save button will not become sensitive until these entries are populated with valid hour values (i.e., numbers from 0 to 23).

## License

[GNU GPLv3](https://choosealicense.com/licenses/gpl-3.0/)
