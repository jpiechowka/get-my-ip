# Get my IP

Get my IP is a small utility to fetch your public IP address using https://api.ipify.org

## How to use

1. Install or build the binary
2. Then just run `get-my-ip`. It will print your IP as (example) `123.123.123.123` without any additional output if the API call was successful.

## Building

`git clone https://github.com/jpiechowka/get-my-ip`

`cd get-my-ip`

`cargo build --release`

## Installing

`git clone https://github.com/jpiechowka/get-my-ip`

`cd get-my-ip`

`cargo install --locked --path .`
