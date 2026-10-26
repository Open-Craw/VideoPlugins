# VideoPlugins

Plugins for [VideoCraw](https://github.com/Open-Craw/VideoCraw), the tool that extracts data from video: image, sound, text, context and search.

VideoCraw's core does very little itself (it reads recipes, opens a video, extracts audio and keeps the results). Everything that finds something in the media is a plugin: a separate program that VideoCraw starts when a recipe calls it. A plugin can be written in any language. This repository holds the plugins that are developed alongside VideoCraw, and the documentation of how to write one.

This repository is in early development. Work happens on the `dev` branch; `main` waits for a first release.

## Writing a plugin

Read [docs/plugin-protocol.md](docs/plugin-protocol.md): the manifest (`plugin.json`), the messages a plugin exchanges with VideoCraw, and an example exchange. A plugin needs nothing from this repository to work; it only has to speak the protocol.

Then [docs/writing-good-plugins.md](docs/writing-good-plugins.md): guidance, not rules, on what makes a plugin pleasant to use in a workflow (report failures as failures, handle `cancel`, stay predictable).

## What is here

| Plugin | Language | What it does |
|---|---|---|
| [speech-to-text](apps/speech-to-text) | Python | writes down the sentences said, with their times (faster-whisper; can be run a window at a time) |
| [sound-volume](apps/sound-volume) | Rust | how loud a sound is over time, with the low, mid and high bands, and where it is silent |
| [sound-graph](apps/sound-graph) | Rust | draws the waves of a sound as an SVG, with the sentences marked |
| [sound-clean](apps/sound-clean) | Rust | takes steady noise out of a sound and hands the cleaned sound to the next plugin |
| [thumbnail](apps/thumbnail) | Rust | chooses the pictures that show a video best (sharp, well exposed, not all from one scene) |
| [example-hits](apps/example-hits) | Rust | the reference plugin: pretends to find things |

Shared code: [libs/plugin-sdk](libs/plugin-sdk) speaks the protocol for a Rust plugin (answers `hello` with its capabilities, runs every call on its own thread, cancel, progress, the work folder), [libs/sound-reader](libs/sound-reader) reads the sound VideoCraw decoded (or any common audio file) and writes a sound for the next plugin, and [libs/frame-reader](libs/frame-reader) reads the pictures VideoCraw decoded. The speech plugin has the same in Python.

## Layout

```
apps/
  <plugin-name>/          one folder per plugin, named as in its plugin.json
    plugin.json           the manifest
    README.md             what it finds, its parameters, what it needs installed
    ...                   its own code, in its own language
docs/
  plugin-protocol.md      the protocol, version 2
  writing-good-plugins.md guidance for plugin authors
```

The repository is a [MonoSmith](https://github.com/monosmith-app/monosmith) workspace, so each plugin is a smith project (`smith add app <name> --lang <language>`) and `smith lint`, `smith test` and `smith build` run for all of them. Give VideoCraw the `apps` folder with `--plugins apps`.

[apps/example-hits](apps/example-hits) is a reference plugin in Rust that pretends to find things; copy it to start your own.

A plugin brings its own runtime and dependencies, or documents them in its README. Nothing in one plugin's folder may depend on another plugin's folder.

Every decision, idea and task is a GitHub issue in this repository.
