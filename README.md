# VideoPlugins

Plugins for [VideoCraw](https://github.com/Open-Craw/VideoCraw), the tool that extracts data from video: image, sound, text, context and search.

VideoCraw's core does very little itself (it reads recipes, opens a video, extracts audio and keeps the results). Everything that finds something in the media is a plugin: a separate program that VideoCraw starts when a recipe calls it. A plugin can be written in any language. This repository holds the plugins that are developed alongside VideoCraw, and the documentation of how to write one.

This repository is in early development. Work happens on the `dev` branch; `main` waits for a first release.

## Writing a plugin

Read [docs/plugin-protocol.md](docs/plugin-protocol.md): the manifest (`plugin.json`), the messages a plugin exchanges with VideoCraw, and an example exchange. A plugin needs nothing from this repository to work; it only has to speak the protocol.

## Layout

```
apps/
  <plugin-name>/          one folder per plugin, named as in its plugin.json
    plugin.json           the manifest
    README.md             what it finds, its parameters, what it needs installed
    ...                   its own code, in its own language
docs/
  plugin-protocol.md      the protocol, version 1
```

The repository is a [MonoSmith](https://github.com/monosmith-app/monosmith) workspace, so each plugin is a smith project (`smith add app <name> --lang <language>`) and `smith lint`, `smith test` and `smith build` run for all of them. Give VideoCraw the `apps` folder with `--plugins apps`.

[apps/example-hits](apps/example-hits) is a reference plugin in Rust that pretends to find things; copy it to start your own.

A plugin brings its own runtime and dependencies, or documents them in its README. Nothing in one plugin's folder may depend on another plugin's folder.

Every decision, idea and task is a GitHub issue in this repository.
