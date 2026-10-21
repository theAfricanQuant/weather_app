# Rust Weather App Resources

## Knowledge

- [Video: I Stopped Watching Rust Tutorials and Built This Weather App Instead](https://www.youtube.com/watch?v=z6_rRxLwu0Y) — the course's walkthrough and timestamps. Use it to see the author navigate the app and narrate the design.
- [Reference repository README](https://github.com/feenix100/rusty_weather#readme-ov-file) — describes the cyber-themed desktop app, features, and run command.
- [Cargo manifest](https://github.com/feenix100/rusty_weather/blob/main/Cargo.toml) — the project's actual edition and dependency features. Check this before assuming an API or version.
- [Rust Book: modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope.html) — use when following <code>main.rs</code> into <code>src/</code> modules.
- [Rust Book: enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) — use for <code>LocationQuery</code>, <code>InputMode</code>, and <code>AnimationMode</code>.
- [Rust Book: recoverable errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) — use when reading input validation and service error paths.
- [Rust Book: threads](https://doc.rust-lang.org/book/ch16-01-threads.html) and [message passing](https://doc.rust-lang.org/book/ch16-02-message-passing.html) — use for <code>thread::spawn</code>, moved request data, and the <code>mpsc</code> channel.
- [Rust standard library: <code>mpsc::channel</code>](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html) — exact sender/receiver behavior used by the worker.
- [eframe <code>run_native</code>](https://docs.rs/eframe/latest/eframe/fn.run_native.html) — use to understand how <code>main.rs</code> starts the native window.
- [egui docs](https://docs.rs/egui/latest/egui/) — use when following immediate-mode UI calls in <code>app.rs</code>.
- [reqwest blocking module](https://docs.rs/reqwest/latest/reqwest/blocking/) — the project enables reqwest's <code>blocking</code> feature and calls it from a worker thread.
- [Serde derive guide](https://serde.rs/derive.html) — use for the <code>Deserialize</code> API structs and serializable history models.
- [Chrono <code>NaiveDate</code>](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html) — use for date-only query values without a timezone.
- [Open-Meteo Forecast API](https://open-meteo.com/en/docs) and [Geocoding API](https://open-meteo.com/en/docs/geocoding-api) — use to check request parameters and response fields at the API boundary.
- [Rust <code>csv</code> crate](https://docs.rs/csv/latest/csv/) — use when following history export.

## Source map

- [<code>main.rs</code>](https://github.com/feenix100/rusty_weather/blob/main/src/main.rs) boots the desktop window.
- [<code>models.rs</code>](https://github.com/feenix100/rusty_weather/blob/main/src/models.rs) names the app's requests, results, and history.
- [<code>app.rs</code>](https://github.com/feenix100/rusty_weather/blob/main/src/app.rs) owns UI state, input validation, worker messages, and rendering.
- [<code>geocoding_service.rs</code>](https://github.com/feenix100/rusty_weather/blob/main/src/geocoding_service.rs) turns a US city/state input into coordinates.
- [<code>weather_service.rs</code>](https://github.com/feenix100/rusty_weather/blob/main/src/weather_service.rs) requests forecast/archive data and maps it into internal records.
- [<code>history_service.rs</code>](https://github.com/feenix100/rusty_weather/blob/main/src/history_service.rs) loads, saves, clears, and exports history.
- [<code>date_utils.rs</code>](https://github.com/feenix100/rusty_weather/blob/main/src/date_utils.rs) centralizes date parsing and formatting.

## Wisdom

No community is required for this course. If the learner wants feedback on a later feature, the [Rust Users Forum](https://users.rust-lang.org/) is a high-signal place to ask a focused question with a minimal code example.

## Gaps

- The video gives a guided walkthrough, not a step-by-step transcript of every code edit. Lessons pair its chapter links with direct source links and independent practice.
- The repository does not provide a platform-specific troubleshooting guide in its short README. If a native build fails, use the exact compiler diagnostic and the relevant eframe/platform documentation.
