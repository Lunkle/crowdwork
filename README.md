# crowdwork
Decentralized, low-profile, and secure platform for executing background tasks, with a reward-based system that incentivizes workers to efficiently process tasks, making it a reliable and scalable solution for distributed computing needs.

## .gitignore
A `.gitignore` file is used to specify which files and directories should be ignored by Git. This helps in keeping the repository clean by not including unnecessary files and directories, such as build artifacts, temporary files, and other files that are not relevant to the project.

The `.gitignore` file for this project includes Rust-specific ignore rules as well as general ignore rules for common files and directories.

## Running the Program
To run the program, follow these steps:

1. Ensure you have Rust installed on your system. If not, you can install it from [here](https://www.rust-lang.org/tools/install).

2. Clone the repository:
   ```sh
   git clone https://github.com/Lunkle/crowdwork.git
   cd crowdwork
   ```

3. Build the project:
   ```sh
   cargo build
   ```

4. Run the program:
   ```sh
   cargo run
   ```

## Specifying the Port for Printing
By default, the program sends user sentences to port `8000` for printing. If you want to specify a different port, you can modify the `addr` variable in the `send_to_port` function in the `src/main.rs` file.
