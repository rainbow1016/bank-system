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
1. Create multiple working threads.
2. Initialize locks the proper way and destroy them when done.
3. Understand how to keep critical sections as small as possible.
4. Prevent deadlocks.

## Video Presentation
[Link to video](https://drive.google.com/file/d/1Ti4ovvphwQYQYgczclwkTAH4yluc6t_Z/view?usp=sharing)

## License
[MIT License](https://opensource.org/licenses/MIT)