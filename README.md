# systemd-compose

Run [compose spec](https://github.com/compose-spec/compose-spec) files (i.e. [`docker-compose`](https://github.com/docker/compose) files) with Podman with a [systemd](https://systemd.io/) service for each compose service.

This will allow you to take a docker-compose project, and expose it to all the management capabilities of systemd. This lets you have `docker-compose` or `podman-compose` for a development or testing tool, but systemd as the orchestrator for a production system.

Rather than being a drop-in replacement for the `docker-compose` command, like [`podman-compose`](https://github.com/containers/podman-compose) attempts to do, this project aims to be a [_systemd generator_](https://www.freedesktop.org/software/systemd/man/systemd.generator.html) that converts compose files to systemd unit files on boot, and on reconfiguration.

## Why Podman?

The goal of this project is not explicitly to use Podman, but Podman is more ready to play nice with systemd due to its forking, rather than client-server architecture. Podman has `podman generate systemd` to generate systemd unit files from existing containers, but this is more of a script, and is not very extensible.

It is possible that this project may be extended to support a Docker backend as well, but I anticipate using only Podman at first. Eventually I may also try to implement `systemd-nspawn` as a backend, but I will prioritize compose spec compliance with Podman first.

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

# Roadmap
Here is a rough outline and checklist of the features that I want to implement, and the priority with which I want to implement them.

## Initial Development Priorities
My first priority is to implement the bare minimum of compose spec features to begin testing this idea. Then I plan to add more compose spec features, and work on the usability of this project coming from `docker-compose` or `podman-compose`.

### Proof of Concept
- Compose file support:
    - [ ] [`image`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#image)
    - [ ] [`entrypoint`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#entrypoint)
    - [ ] [`command`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#command)
- Service file generation:
    - [ ] Podman container start
    - [ ] Podman container creation

### Minimum Viable Product
- Compose file support:
    - [ ] [`networks`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#networks)
    - [ ] [`network_mode`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#network_mode)
    - [ ] [`container_name`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#container_name)
    - [ ] [`ports`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#ports)
    - [ ] [`volumes`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#volumes)
    - [ ] [`depends_on`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#depends_on)
    - [ ] [`env_file`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#env_file)
    - [ ] [`environment`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#environment)
    - [ ] [`expose`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#expose)
- Service file generation:
    - [ ] Podman bridged network creation
- CI:
    - [ ] amd64 binaries release to GitHub
    - [ ] arm64 binaries release to GitHub

## Compose Spec Compliance Roadmap
Eventually, I would like as much compliance with the compose spec as is practicable for systemd (single host systems).

### More Lifecycle Management
- [ ] [`restart`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#restart)
- [ ] [`restart_policy`](https://github.com/compose-spec/compose-spec/blob/master/deploy.md#restart_policy)

### Compose file flexibility
- [ ] [`extends`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#extends)
- [ ] Environment variable interpolation

### Add Resource Control
- [ ] [`deploy.resources.limits.cpus`](https://github.com/compose-spec/compose-spec/blob/master/deploy.md#cpus)
- [ ] [`deploy.resources.limits.memory`](https://github.com/compose-spec/compose-spec/blob/master/deploy.md#memory)
- [ ] [`cpuset`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#cpuset)

### Majority Use Case Compose Spec Support
- [ ] `devices`
- [ ] `ipc`
- [ ] `init`
- [ ] `labels`
- [ ] [`runtime`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#runtime)

### Compose Spec Compliance
- [ ] `isolation`
- [ ] `group_add`
- [ ] `links`
- [ ] `logging`
- [ ] [`build`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#build)
- [ ] [`cgroup_parent`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#cgroup_parent)
- [ ] [`configs`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#configs)
- [ ] [`credential_spec`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#credential_spec)
- [ ] [`device_cgroup_rules`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#device_cgroup_rules)
- [ ] [`dns`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#dns)
- [ ] [`dns_opt`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#dns_opt)
- [ ] [`dns_search`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#dns_search)
- [ ] [`domainname`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#domainname)
- [ ] [`healthcheck`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#healthcheck)
- [ ] [`cap_add`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#cap_add)
- [ ] [`cap_drop`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#cap_drop)
- [ ] [`external_links`](https://github.com/compose-spec/compose-spec/blob/master/spec.md#external_links)
- [ ] `extra_hosts`
- etc.

## `systemd-compose` script wrapper
To make this project easier to use, I would eventually like to create a script wrapper that helps with setup, and maybe even mimics functionality of `docker-compose` or `podman-compose`.

- [ ] Add `install` verb that copies compose files into the generator's directory
- [ ] Add `up` verb that installs, and then immediately does a `systemctl enable` and `systemctl start`
- [ ] Add `down` verb that does `systemctl stop`, `systemctl disable`, and uninstalls compose files