# systemd-compose

Run [compose spec](https://github.com/compose-spec/compose-spec) files (i.e. [`docker-compose`](https://github.com/docker/compose) files) with Podman with a [systemd](https://systemd.io/) service for each compose service.

This will allow you to take a docker-compose project, and expose it to all the management capabilities of systemd. This lets you have `docker-compose` or `podman-compose` for a development or testing tool, but systemd as the orchestrator for a production system.

Rather than being a drop-in replacement for the `docker-compose` command, like [`podman-compose`](https://github.com/containers/podman-compose) attempts to do, this project aims to be a [_systemd generator_](https://www.freedesktop.org/software/systemd/man/systemd.generator.html) that converts compose files to systemd unit files on boot, and on reconfiguration.

## Why Podman?

The goal of this project is not explicitly to use Podman, but Podman is more ready to play nice with systemd due to its forking, rather than client-server architecture. Podman also has a nifty `podman generate systemd` functionality to generate systemd unit files from existing containers, which may be used or referenced in this project.

It is possible that this project may be extended to support a Docker backend as well, but I anticipate using only Podman at first.

## The Dream Workflow

- Install `systemd-generator` (by moving its binary to `/usr/lib/systemd/system-generators` or another generator directory)
- Install `docker-compose.yml` files to `/etc/systemd-compose/`
- Reboot system, or call `systemctl daemon-reload`
- Have all your containerized services start, and be visible from `systemctl`

# Choice of how to parse compose files
While there are small projects that implement Serde for the compose spec like 
https://github.com/emk/compose_yml and https://github.com/stephanbuys/docker-compose-types, they do not support the latest compose spec, are not highly maintained, and are more focused on programmatic usability.

Important: for this generator, we don't care about usability in generating compose files programmatically. We just want to be able to read, validate, and translate them.

The compose spec is published as a JSON schema in addition to a document. We want to stay as close to this JSON schema as possible (https://github.com/compose-spec/compose-spec/blob/master/schema/compose-spec.json).

Backwards compatibility with previous compose spec versions (notably Compose v2) is not a priority.

# Why serde for systemd then?
There is no official machine-readable schema for systemd that I could find, so I created one at https://github.com/Aposhian/systemd-schema.