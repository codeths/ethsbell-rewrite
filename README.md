# ETHSBell Rewrite

This is a rewrite of ETHSBell in Rust. It features...
* Restored support for arbitrary dates and times.
* A strongly typed API with stability guarantees.
  * Behavior under `/api` won't have any breaking changes, ever.
  * The two most recent API versions will be present.
* Many endpoints for different purposes.
  * `GET /api/v1/schedule` returns the whole runtime schedule struct for local use.
  * `GET /api/v1/today` returns today's schedule.
  * `GET /api/v1/today/now` returns the current period.
  * `GET /api/v1/today/at` returns the period for an arbitrary time of day formatted like `HH:MM:SS`.
  * `GET /api/v1/on/<date>` returns the schedule for an arbitrary date formatted like `YYYY-MM-DD`.
  * `GET /api/v1/on/<date>/at/<time>` returns the period for an arbitrary date and time formatted like `YYYY-MM-DD` and `HH:MM:SS`.
* In-memory caching.
  * You can expect 3-4MB of memory usage when idle.
* Stateless design.
* Runtime-less deployment.
* Native performance.
* Rust library for client-side processing.
  * Client-side processing in WASI is blocked by [Rocket async](https://github.com/SergioBenitez/Rocket/projects/1), so it's not possible to do just yet.