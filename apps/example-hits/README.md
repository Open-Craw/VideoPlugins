# example-hits

A reference plugin. It pretends to find something in a video, so recipes and hosts can be tried and tested without a model. Copy it to start a plugin of your own: [src/serve](src/serve) is the whole protocol, [src/methods](src/methods) is the work.

## Methods

| Method | Params | Result |
|---|---|---|
| `find` | `durationMs`, `everyMs` (default 1000), `lengthMs` (default 500), `label` (default `hit`) | `{ "hits": [{ "startMs", "endMs", "label" }] }`, one hit every `everyMs`, the last cut at the end of the video |
| `size` | `audio`: a stored file as VideoCraw passes it (an object with an absolute `file`) | `{ "bytes": <size of the file> }` |
| `sleep` | `ms` | waits that long and answers `{ "waitedMs" }`; answers the error `cancelled` as soon as the call is cancelled |
| `pid` | none | `{ "pid" }`, the number of the program's process, so a host can tell copies apart |
| `fail` | none | always an error |
| `crash` | none | the program exits without answering |
| `hang` | none | never answers, and ignores a cancel |

`crash`, `hang` and `fail` exist so a host can test how it copes with a plugin that misbehaves. Every call runs on its own thread, so the plugin takes as many calls at once as the host sends: its `plugin.json` can declare a `concurrency` above 1.

## Build and use

```sh
smith build -p example-hits
videocraw run recipes/ --out out/result.json --plugins apps
```

`plugin.json` starts the release build in `dist/cargo/release`, relative to this folder. This plugin needs nothing installed beyond the binary.
