[![Build Status](https://travis-ci.org/ThomasdenH/rust-openttd-admin.svg?branch=master)](https://travis-ci.org/ThomasdenH/rust-openttd-admin)
[![Coverage Status](https://coveralls.io/repos/github/ThomasdenH/rust-openttd-admin/badge.svg?branch=master)](https://coveralls.io/github/ThomasdenH/rust-openttd-admin?branch=master)

# WIP: rust-openttd-admin

This crate aims to be a type safe interface to the OpenTTD Admin API. It is very much a work in progress so a lot of changes will happen.

It aims to offer functionality on multiple levels:

- Packets: Basic packet reading and building.
- Sockets: Abstractions that make it easier to setup the connection correctly.
- Game state: Storage of historical data and game state.
