# bank-system-rust

## Table of Contents

- [Project Description](#project-description)
- [Learning Objectives](#learning-objectives)
- [Video Presentation](#video-presentation)

## Project Description

This project is an extension of Project 4: Prosumer/Consumer with different implementations and additional functionality. The goal of this project is to implement a simple banking application with Rust. 
Particularly, there is multiple accounts in the bank and each account has a balance where we can deposit, withdraw, or transfer money between accounts.

## Design Decisions and Functionality
- The project is implemented with Rust which has several advantages over C++. For example, Rust provides the memory safety through ownership and borrowing rule, eliminating many common programmings errorsm such as null pointer dereferences. Rust also has a lightweight concurrency model (referred as "fearless concurrency")that makes it easy to write concurrent programs without introducing data races or deadlocks, providng a safe and write .
- The program will initialize a bank with 10 accounts. Then, it loads a text file containing the information of the ledger and creates "num_workers" (input from terminal) threads to process the transactions in the ledger list concurrently. Finally, the program will print the balance of all the accounts in the bank.

## Learning Objectives
1. Create multiple concurrent tasks using Rust's built-in threading capabilities.
2. Apply Rust's ownership and borrowing system to minimize the scope and duration of critical sections, reducing the likelihood of data races.
3. Employ Rust's synchronization primitives, such as Mutex and RwLock, to prevent deadlocks and ensure thread-safe execution.

## Installation

1. Make sure you have Rust and Cargo installed. If not, follow the official Rust installation guide: [Rust Installation Guide](https://www.rust-lang.org/tools/install).

2. Clone the repository:

    ```shell
    git clone https://github.com/ThomasN12/bank-system-rust
3. Navigate the project directory:
    ```shell
    cd bank-system
3. Build the project using Cargo:
    ```shell
    cargo build
4. Run the project with the following command:
    ```shell
    cargo run -- <num_workers> <ledger_filename>

## Video Presentation
1. [Link to video](https://youtu.be/nEB7b6ynsek)
2. [Link to video-2](https://drive.google.com/file/d/1Ti4ovvphwQYQYgczclwkTAH4yluc6t_Z/view?usp=sharing) (In case the first one not working)

## Slides
[Link to slide](https://docs.google.com/presentation/d/1HH4uaMCePURDQnES0Itl3q2Dm7TtbcZ9UFhD-bga04s/edit?usp=sharing)

## License
[MIT License](https://opensource.org/licenses/MIT)