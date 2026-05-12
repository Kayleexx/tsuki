# Tsuki

Tiny self hosted deployment platform built in Rust.

Tsuki builds container images locally, ships them over SSH, runs them remotely with Docker, and automatically configures reverse proxy routing using Caddy.

## Features

- Automatic app detection
- Local Docker image builds
- Artifact export and transfer over SSH
- Remote Docker image loading
- Automatic container deployment
- Automatic port allocation
- Reverse proxy configuration with Caddy
- Health checks after deployment
- Deployment history tracking with SQLite
- Runtime status inspection
- Automatic `sslip.io` domain routing
- Rollback foundation

## Architecture

```text
Developer
    ↓
Tsuki CLI
    ↓ SSH
Remote Host
 ├── Docker
 ├── Caddy
 └── SQLite metadata
````

## Requirements

### Local machine

* Rust
* Docker
* OpenSSH client

### Remote machine

* Docker
* Caddy
* OpenSSH server

## Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/tsuki.git
cd tsuki
```

Build the project:

```bash
cargo build
```

## Remote Host Setup

Install Docker:

```bash
sudo apt install docker.io
sudo systemctl enable --now docker
```

Install Caddy:

```bash
sudo apt install caddy
sudo systemctl enable --now caddy
```

Install OpenSSH server:

```bash
sudo apt install openssh-server
sudo systemctl enable --now ssh
```

Add passwordless sudo rules for Tsuki:

```bash
sudo visudo -f /etc/sudoers.d/tsuki
```

Add:

```text
your-user ALL=(ALL) NOPASSWD: /usr/bin/tee, /usr/bin/systemctl, /usr/bin/caddy
```

## Example App

Example `Dockerfile`:

```dockerfile
FROM python:3.12-alpine

WORKDIR /app

RUN echo 'hello from tsuki' > index.html

CMD ["python", "-m", "http.server", "80"]
```

## Deploying

From inside your app directory:

```bash
cargo run -- deploy .
```

Example output:

```text
✓ Starting deployment
✓ Building container
✓ Uploading artifact
✓ Loading remote image
✓ Allocating port 8000
✓ Configuring reverse proxy
✓ Running health checks

Application live at:
https://test-app.192.168.0.47.sslip.io
```

## Deployment History

Deployments are stored in SQLite.

Example:

```bash
sqlite3 ~/.tsuki/tsuki.db
```

```sql
SELECT * FROM deployments;
```

## Status Inspection

```bash
cargo run -- status test-app
```

Example:

```text
APP:        test-app
STATUS:     running
PORT:       8000
IMAGE:      tsuki-app:1778533905
CONTAINER:  abc123
```

## Rollback

```bash
cargo run -- rollback test-app
```

## Current Limitations

* Single host deployments
* Single Caddyfile configuration
* No authentication layer
* No orchestration or clustering
* No automatic scaling

## demo

<img width="958" height="881" alt="image" src="https://github.com/user-attachments/assets/2dcce2ce-ddcd-4887-839b-e43062337797" />


<img width="407" height="65" alt="image" src="https://github.com/user-attachments/assets/8ee56922-b6d2-4973-b9e2-2f16a44a1b18" />


