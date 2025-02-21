
# Table of Contents

    -   [Overview](#org76078f4)
    -   [Features](#org4c0f4c8)
    -   [Installation](#org4670684)
    -   [Usage](#org60827d2)
        -   [Register a Behavior](#org120697e)
        -   [Fetch a Behavior](#org26d616d)
-   [API Reference](#orgc18fd26)
    -   [POST /api/v2/pop-server/register-behavior](#orgb10fbce)
    -   [Dynamic Behavior Matching](#org4ab6bcd)
    -   [License](#orga548a0c)
    -   [Author](#orgc0f1c9b)

![img](https://raster.shields.io/badge/Rust-v1.84-b7410e.png) ![img](https://raster.shields.io/badge/Nix-Flake-1f5cd6.png)

pop-server - a puppet server, at your command.

This a web-server that helps you simulate behaviours and test integrations with external systems.


<a id="org76078f4"></a>

## Overview

`pop-server` is a lightweight HTTP server that allows dynamic behavior registration and retrieval. It acts as a simple puppet server where you can define request-response behaviors dynamically via HTTP requests. This can be useful for testing, mocking APIs, or creating programmable server responses.


<a id="org4c0f4c8"></a>

## Features

-   Register custom behaviors dynamically via HTTP headers and request bodies.
-   Fetch stored behaviors based on incoming requests.
-   Supports custom HTTP methods, paths, content types, and response statuses.
-   Built with **Rust**, using **Hyper** for HTTP handling and **Tokio** for async networking.


<a id="org4670684"></a>

## Installation

You can find releases at:

crates.io: <https://crates.io/crates/pop-server>
dockerhub: <https://hub.docker.com/repository/docker/jjba23/pop-server/general>

A Nix flake is provided at the root of this repo.

If you don't use Nix, ensure you have Rust installed. You can install Rust via rustup: <https://rustup.rs/>

The project uses Cargo to build Rust code and [Makefile](Makefile) for automating tasks and the build processes. 


<a id="org60827d2"></a>

## Usage

By default, the server listens on `http://127.0.0.1:50088`

It can be useful to use different logging levels, with the `RUST_LOG` environment variable, e.g. `debug`, `info`

You can start pop-server with certain environment variables to override behaviors:

`POP_SERVER_LISTEN_PORT` - HTTP port where pop-server listens, defaults to `50088`

`POP_SERVER_REGISTER_ROUTE` - Route where behaviors are registered, defaults to `/api/v2/pop-server/register-behavior`


<a id="org120697e"></a>

### Register a Behavior

To register a new behavior (in memory), send a **POST** request to the register route, by default `/api/v2/pop-server/register-behavior` , with custom headers and a body:

    curl -X POST "http://127.0.0.1:50088/api/v2/pop-server/register-behavior" \
       -H "content-type: application/json" \       
       -H "x-pop-server-request-method: GET" \
       -H "x-pop-server-request-path: /test" \
       -H "x-pop-server-response-content-type: application/json" \
       -H "x-pop-server-response-status: 200" \
       --data '{"message": "Hello, world!"}'


<a id="org26d616d"></a>

### Fetch a Behavior

Once a behavior is registered (in-memory), you can retrieve it by making a request matching the registered method and path:

    curl -X GET "http://127.0.0.1:50088/test"

If a match is found, the stored behavior is returned; otherwise, a `404 Not Found` response is given.


<a id="orgc18fd26"></a>

# API Reference


<a id="orgb10fbce"></a>

## POST /api/v2/pop-server/register-behavior

Endpoint to register behaviors.

**Headers:**

-   `x-pop-server-request-method`: HTTP method (e.g., `GET`, `POST`, etc.)
-   `x-pop-server-request-path`: Path to match (e.g., `/test`)
-   `x-pop-server-response-content-type`: Content-type of response
-   `x-pop-server-response-status`: HTTP status code (default: `200`)
-   `x-pop-server-request-content-type`: Optional, Content-type of the incoming request, useful for e.g. POST

**Body:**

-   JSON or any other valid body content that you wish to later return.


<a id="org4ab6bcd"></a>

## Dynamic Behavior Matching

If an incoming request matches a registered behavior (method & path), the server responds with the stored content.

Otherwise, it returns a `404 Not Found`.


<a id="orga548a0c"></a>

## License

`pop-server` is licensed under the **Lesser GNU General Public License v3.0**. See the COPYING file or <https://www.gnu.org/licenses/lgpl-3.0.html> for more details.


<a id="orgc0f1c9b"></a>

## Author

Created by **Josep Bigorra (jjba23)**. For contributions, issues, or questions, contact: <mailto:jjbigorra@gmail.com>.

