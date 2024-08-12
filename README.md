# private-automation-server

Designed for small projects to build quickly!

## Goals

`private-automation-server` doesn't aim to:

- be a full-fledged CI/CD server
- create a new language for writing scripts
- reinvent the wheel
- make a grilled cheese
- create reproducible builds
- be safe for production at all
- have 12 nine's of uptime

though, it will try to:

- be very simple to use
- extend and deploy easily
- have a nice web interface

<details>

<summary>WIP usage</summary>

## Usage

Using the binary on bare metal is an anti-pattern. Instead, use Docker!

At the moment, the Docker image tag is `latest` and is based on `alpine:latest`.
This is subject to change, just open an issue if you have needs that cannot be satisfied by alpine.

Create a `Dockerfile`:

```Dockerfile
FROM pub.sup.ply.how/auto:latest
```

</details>
