# Plugin protocol

A plugin is a separate program that finds something in the media (or delivers a result). VideoCraw never runs plugin code inside itself: it starts the program when a recipe calls it, talks to it in JSON lines, and stops it when the run ends. A plugin can be written in any language. This page is protocol version **1**, the first slice.

## Installing

A plugin is a folder with a `plugin.json`. VideoCraw is given a folder that holds plugin folders, or one plugin's folder, with `--plugins` (as often as needed):

```sh
videocraw run recipes/ --out out/result.json --plugins ~/.videocraw/plugins
```

```json
{
  "name": "cat-sounds",
  "version": "0.1.0",
  "protocol": 1,
  "effect": "pure",
  "command": ["python", "main.py"],
  "timeoutSeconds": 600
}
```

| Key | Meaning |
|---|---|
| `name` | what recipes call it (unique among installed plugins) |
| `protocol` | the protocol version it speaks; VideoCraw refuses any other |
| `effect` | `pure` (same input, same output: may be cached and retried later) or `effectful` (changes something outside). Recorded now, not used yet |
| `command` | the program and its arguments. A first entry with a path separator is relative to the plugin folder; a bare name is looked up on the system path. The program runs with the plugin folder as its working directory |
| `timeoutSeconds` | how long one call may take (default 600). After that the plugin is stopped and the video fails |

Unknown keys are refused. VideoCraw does not install a plugin's runtime or dependencies: the plugin brings its own or documents them.

## Using one in a recipe

```json
{ "type": "plugin", "id": "found", "plugin": "cat-sounds", "method": "find",
  "params": { "audio": "{{ audio }}", "minConfidence": 0.6 } }
```

`method` defaults to `run`. Text values in `params` may use `{{ path }}`; a single `{{ path }}` passes the value as it is. A stored file (such as the result of an `audio` step) is passed as its object with `blob`, `mime`, `path` and `bytes`, plus `file`, the **absolute path** the plugin can open. What the plugin answers is kept under the step's id, so a recipe's `forEach` can turn a list in it into records. Times are integer milliseconds. A plugin used by any recipe must be installed: the run stops before it starts when one is missing. A plugin starts on its first call and stays up for the rest of the run.

## The protocol

One JSON object per line. VideoCraw writes to the plugin's standard input and reads its standard output; the plugin's standard error goes to VideoCraw's, so it is the place for logs. Anything else on standard output is a protocol error.

| Direction | Message | |
|---|---|---|
| core to plugin | `{"type":"hello","protocol":1}` | first message |
| plugin to core | `{"type":"ready","protocol":1}` | the answer, within 30 s |
| core to plugin | `{"type":"call","id":1,"method":"find","params":{...}}` | one call at a time |
| plugin to core | `{"type":"result","id":1,"result":{...}}` | the answer to call 1 |
| plugin to core | `{"type":"error","id":1,"message":"..."}` | the call failed; the plugin stays up and the video fails |
| plugin to core | `{"type":"progress","id":1,"message":"..."}` | optional while a call runs; ignored for now |
| core to plugin | `{"type":"shutdown"}` | finish and exit (VideoCraw stops it after 5 s) |

A plugin that times out, exits, or says something the protocol does not allow is stopped; the next call starts a fresh process. Unknown fields in a plugin's messages are ignored, so a newer plugin can say more than VideoCraw understands.

### Example exchange

```
core   -> {"type":"hello","protocol":1}
plugin -> {"type":"ready","protocol":1}
core   -> {"type":"call","id":1,"method":"find","params":{"audio":{"blob":"sha256:9f2c…","mime":"audio/mpeg","path":"blobs/9f/9f2c….mp3","bytes":2048,"file":"C:/work/out/blobs/9f/9f2c….mp3"},"label":"cat"}}
plugin -> {"type":"result","id":1,"result":{"hits":[{"startMs":12330,"endMs":13100,"label":"cat"}]}}
core   -> {"type":"shutdown"}
```

### A minimal plugin

In any language, the whole job is: read lines, answer lines. In pseudo-code:

```
for each line on standard input:
    message = parse JSON
    if message.type == "hello":    write {"type":"ready","protocol":1}
    if message.type == "call":     write {"type":"result","id":message.id,"result":<your answer>}
                                   (or {"type":"error","id":message.id,"message":"why"})
    if message.type == "shutdown": exit
flush standard output after every line
```

## Not in this version

Shared-memory regions and writable regions for large data, host services (the plugin asking VideoCraw for frames or clips), typed ports and the check that a step's output fits the next input, sockets and WebSocket as transports, permissions and signing, install and update commands, and caching of `pure` plugins.
