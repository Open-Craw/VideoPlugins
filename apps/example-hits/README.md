# example-hits

A reference plugin. It pretends to find something in a video, so recipes and hosts can be tried and tested without a model. Copy it to start a plugin of your own: [src/serve](src/serve) is the whole protocol, [src/methods](src/methods) is the work.

## Methods

| Method | Params | Result |
|---|---|---|
| `find` | `durationMs`, `everyMs` (default 1000), `lengthMs` (default 500), `label` (default `hit`) | `{ "hits": [{ "startMs", "endMs", "label" }] }`, one hit every `everyMs`, the last cut at the end of the video |
| `size` | `audio`: a stored file as VideoCraw passes it (an object with an absolute `file`) | `{ "bytes": <size of the file> }` |
| `fail` | none | always an error |
| `crash` | none | the program exits without answering |
| `hang` | none | never answers |

The last three exist so a host can test how it copes with a plugin that misbehaves.

## Build and use

```sh
smith build -p example-hits
videocraw run recipes/ --out out/result.json --plugins apps
```

`plugin.json` starts the release build in `dist/cargo/release`, relative to this folder. This plugin needs nothing installed beyond the binary.
