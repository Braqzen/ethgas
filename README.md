# Overview

A command line interface written in Rust to interact with some of the API from [EthGas](https://developers.ethgas.com/).

Its commands are tailored to submit a preconf request (bundle).

> NOTE: Incomplete, specifically commands for submission of a preconf request.

# Usage

You must first familiarise yourself with their API so read their [docs](https://developers.ethgas.com/).

## Environment

For a better UX copy [.env.example](./.env.example) and name it `.env` and fill in the fields that you do not want to repeatedly input into the `cli`. The `cli` reads the file and then overrides the environment variables with any arguments you provide.

## Installation

Install [Rust](https://www.rust-lang.org/) and then run

```shell
cargo install --path .
```

Check the installation by displaying a help message

```shell
ethgas --help
```

It should look something like

```shell
$ ethgas -help
Usage: ethgas <COMMAND>

Commands:
  accounts       Check account info
  buy            Buy gas for a market
  deposit        Make a deposit
  deposits       Check deposits
  jwt            Get JWT token
  markets        Check markets
  slot           Check slot
  submit-bundle  Submit transaction bundle
  transfer       Transfer between accounts
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Example

Get the jwt for the other commands

```shell
$ ethgas jwt \
  --base-url <BASE_URL> \
  --chain-id <CHAIN_ID> \
  --private-key <PRIVATE_KEY>
```
