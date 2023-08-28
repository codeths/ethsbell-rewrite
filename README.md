# ETHSBell Rewrite

-   [ETHSBell Rewrite](#ethsbell-rewrite)
    -   [About](#about)
    -   [Development](#development)
    -   [Deployment](#deployment)
        -   [x86 with Docker](#x86-with-docker)
        -   [Non-x86 or without Docker](#non-x86-or-without-docker)
-   [The resulting binary will be placed at ./target/release/ethsbell-rewrite](#the-resulting-binary-will-be-placed-at-targetreleaseethsbell-rewrite)
    -   [Known Issues](#known-issues)

## About

This is a rewrite of ETHSBell in Rust. It features...

-   Restored support for arbitrary dates and times.
-   A strongly typed API with stability guarantees.
    -   Behavior under `/api` won't have any breaking changes, ever.
        -   \*Unless we're still in development, in which case all bets are off...
    -   The two most recent API versions will be present.
-   Many endpoints for different purposes.
    -   `GET /api/v1/schedule` returns the whole runtime schedule struct for local use.
    -   `GET /api/v1/today` returns today's schedule.
    -   `GET /api/v1/today/now` returns the current period.
    -   `GET /api/v1/today/at` returns the period for an arbitrary time of day formatted like `HH:MM:SS`.
    -   `GET /api/v1/on/<date>` returns the schedule for an arbitrary date formatted like `YYYY-MM-DD`.
        -   `GET /api/v1/on/<date>/at/<time>` returns the period for an arbitrary date and time formatted like `YYYY-MM-DD` and `HH:MM:SS`.
            -   In-memory caching.
    -   You can expect 3-4MB of memory usage when idle.
-   Stateless design.
-   Runtime-less deployment.
-   Native performance.
-   Rust library for client-side processing.

_Psst! If you're feeling really crafty, you could even self-host this and use it to structure your free time! For example, you could add an extra "class period" and designate it as homework time._

## Development

_See [CONTRIBUTING](CONTRIBUTING.md) for contribution guidelines._

_Follow the instructions in [DEVELOPING](DEVELOPING.md) to set up a development environment._

## Deployment

### x86 with Docker

You can deploy the software with a Compose file like this.

```yml
version: "3.7"

services:
 web:
  image: ghcr.io/codeths/ethsbell-rewrite/ethsbell-rewrite:latest
  restart: unless-stopped
  init: true
  ports:
    - 8000:8000
  volumes:
  volumes:
   - "/etc/localtime:/etc/localtime:ro"
   - "./advisory.html:/app/frontend-dist/advisory.html"
   - type: bind
     source: ./def.json
     target: /app/def.json
     volume:
       nocopy: true
```

### Non-x86 or without Docker

You can deploy the software by building it from source like this.

```sh
git clone https://github.com/chromezoneeths/ethsbell-rewrite.git
cd ethsbell-rewrite
cargo build
# The resulting binary will be placed at ./target/release/ethsbell-rewrite
```

## Known Issues

-   On some (non-Safari) Webkit-based browsers, using the zoom feature causes the progress wheels to render incorrectly. Browsers based on Blink and Gecko do not have this issue, and the small market share of these malfunctioning browsers means this issue is unlikely to be fixed.
-   Time zones aren't handled correctly right now. If you're in a different time zone (or your browser is reporting a different time zone) you may experience issues using the service, including incorrect schedules and nonsensical (often negative) displayed durations. If you're having issues like these, please make sure your computer is set to Chicago time and your browser isn't hiding your time zone (Firefox's `privacy.resistFingerprinting` flag is known to cause this).

[🪦 Remembering Oliver Leopold](https://ethsbell.app/api/legacy/oliver)
