# Licences of what the plugins are built with

Written 2026-10-26. This page records **what the plugins in this repository use and under which licences, as the packages say about themselves**. It is an inventory, not a legal opinion; the questions that need one are at the end.

## What is where

| What | How it is used | Licence situation |
|---|---|---|
| The Rust crates below | linked into the Rust plugins (`sound-volume`, `sound-graph`, `sound-clean`, `thumbnail`, `example-hits`) | permissive, except the **symphonia** crates (MPL-2.0, see the notes) |
| The Python packages below | installed for `speech-to-text` | permissive, except `tqdm` and `certifi` (MPL-2.0) |
| **FFmpeg** | **not used by any plugin.** VideoCraw decodes the media and hands it over | the plugins carry no decoder of their own except what is said below |
| Speech models | **not packaged.** `speech-to-text` downloads the model you name (tiny by default) from the Hugging Face hub, once | the licence of each model is read from its model card at the place it is downloaded from; the weights of OpenAI Whisper are published under MIT, and the converted ones used by faster-whisper by the people who converted them: to be confirmed for each model before it is recommended |

## Notes

- **symphonia (MPL-2.0).** The sound reader (`libs/sound-reader`) uses it to read mp3, wav, flac, ogg, aac and alac files when a plugin is given a file and not the sound VideoCraw decoded. MPL-2.0 is a file-level copyleft: using the crates unmodified, in a program of another licence, is allowed; the notice and the source of *those files* (not of the plugin) have to be available to whoever gets the program. The plugins do not modify them.
- **av (PyAV, BSD-3-Clause).** `faster-whisper` depends on it to decode audio when it is given a file. Its binary wheels carry FFmpeg libraries, whose own licence (LGPL, or GPL if built with GPL parts) is separate from the BSD licence of the Python wrapper: to be checked for the wheel that is installed before the plugin is distributed. When VideoCraw hands the plugin decoded samples, none of it is used.
- **tqdm and certifi (MPL-2.0).** Same file-level terms as above; used unmodified.
- **Build tools** (ruff, mypy, pytest, and so on) are not part of what a plugin ships and are left out of the lists.

## The packages

Regenerate with `python tools/list_licences.py <this repository> --python <its python> faster-whisper numpy` from the VideoCraw repository (the script is there); it reads what each package says and does not read licence texts.

### Rust crates

- 24 x MIT OR Apache-2.0
- 13 x MPL-2.0
- 2 x Apache-2.0 OR MIT
- 2 x MIT/Apache-2.0
- 2 x MIT
- 1 x Zlib OR Apache-2.0 OR MIT
- 1 x (Apache-2.0 OR MIT) AND BSD-3-Clause
- 1 x Unlicense OR MIT
- 1 x (MIT OR Apache-2.0) AND Unicode-3.0

| Package | Version | Licence |
|---|---|---|
| arrayvec | 0.7.8 | MIT OR Apache-2.0 |
| autocfg | 1.5.1 | Apache-2.0 OR MIT |
| bitflags | 1.3.2 | MIT/Apache-2.0 |
| bytemuck | 1.25.2 | Zlib OR Apache-2.0 OR MIT |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 |
| core_detect | 1.0.0 | MIT/Apache-2.0 |
| encoding_rs | 0.8.42 | (Apache-2.0 OR MIT) AND BSD-3-Clause |
| extended | 0.1.0 | MIT |
| itoa | 1.0.18 | MIT OR Apache-2.0 |
| lazy_static | 1.5.1 | MIT OR Apache-2.0 |
| libc | 0.2.191 | MIT OR Apache-2.0 |
| log | 0.4.34 | MIT OR Apache-2.0 |
| memchr | 2.8.3 | Unlicense OR MIT |
| memmap2 | 0.9.12 | MIT OR Apache-2.0 |
| multiversion_no_op | 1.0.0 | Apache-2.0 OR MIT |
| num-complex | 0.4.6 | MIT OR Apache-2.0 |
| num-integer | 0.1.47 | MIT OR Apache-2.0 |
| num-traits | 0.2.19 | MIT OR Apache-2.0 |
| primal-check | 0.3.4 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| rustfft | 6.4.1 | MIT OR Apache-2.0 |
| rustversion | 1.0.23 | MIT OR Apache-2.0 |
| scopeguard | 1.2.0 | MIT OR Apache-2.0 |
| serde | 1.0.229 | MIT OR Apache-2.0 |
| serde_core | 1.0.229 | MIT OR Apache-2.0 |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 |
| simdutf8 | 0.1.5 | MIT OR Apache-2.0 |
| strength_reduce | 0.2.4 | MIT OR Apache-2.0 |
| symphonia | 0.5.5 | MPL-2.0 |
| symphonia-bundle-flac | 0.5.5 | MPL-2.0 |
| symphonia-bundle-mp3 | 0.5.5 | MPL-2.0 |
| symphonia-codec-aac | 0.5.5 | MPL-2.0 |
| symphonia-codec-alac | 0.5.5 | MPL-2.0 |
| symphonia-codec-pcm | 0.5.5 | MPL-2.0 |
| symphonia-codec-vorbis | 0.5.5 | MPL-2.0 |
| symphonia-core | 0.5.5 | MPL-2.0 |
| symphonia-format-isomp4 | 0.5.5 | MPL-2.0 |
| symphonia-format-ogg | 0.5.5 | MPL-2.0 |
| symphonia-format-riff | 0.5.5 | MPL-2.0 |
| symphonia-metadata | 0.5.5 | MPL-2.0 |
| symphonia-utils-xiph | 0.5.5 | MPL-2.0 |
| syn | 3.0.9 | MIT OR Apache-2.0 |
| transpose | 0.2.3 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| zmij | 1.0.23 | MIT |

### Python packages (speech-to-text: faster-whisper and numpy, and what they need)

- 8 x MIT
- 6 x BSD-3-Clause
- 2 x Apache-2.0
- 1 x BSD License
- 1 x Apache 2.0
- 1 x BSD-3-Clause AND 0BSD AND MIT AND Zlib AND CC0-1.0
- 1 x MIT License
- 1 x Apache-2.0 OR BSD-2-Clause
- 1 x 3-Clause BSD License
- 1 x Apache Software License
- 1 x MPL-2.0 AND MIT
- 1 x PSF-2.0

| Package | Version | Licence |
|---|---|---|
| anyio | 4.15.1 | MIT |
| av | 16.1.0 | BSD-3-Clause |
| click | 8.5.0 | BSD-3-Clause |
| colorama | 0.4.6 | BSD License |
| ctranslate2 | 4.8.3 | MIT |
| faster-whisper | 1.2.1 | MIT |
| filelock | 4.2.4 | MIT |
| flatbuffers | 25.12.19 | Apache 2.0 |
| fsspec | 2026.9.0 | BSD-3-Clause |
| h11 | 0.16.0 | MIT |
| hf-xet | 1.7.0 | Apache-2.0 |
| httpcore2 | 2.16.0 | BSD-3-Clause |
| httpx2 | 2.16.0 | BSD-3-Clause |
| huggingface-hub | 2.3.0 | Apache-2.0 |
| idna | 3.20 | BSD-3-Clause |
| numpy | 2.5.4 | BSD-3-Clause AND 0BSD AND MIT AND Zlib AND CC0-1.0 |
| onnxruntime | 1.31.0 | MIT License |
| packaging | 26.3 | Apache-2.0 OR BSD-2-Clause |
| protobuf | 7.36.2 | 3-Clause BSD License |
| pyyaml | 6.0.3 | MIT |
| tokenizers | 0.23.3 | Apache Software License |
| tomli | 2.5.0 | MIT |
| tqdm | 4.70.1 | MPL-2.0 AND MIT |
| truststore | 0.10.5 | MIT |
| typing-extensions | 4.16.0 | PSF-2.0 |


## Policy for adding a dependency to a plugin

The same as for VideoCraw: permissive licences (MIT, Apache-2.0, BSD, ISC, Zlib, Unlicense, CC0, Unicode), or a written decision before anything else goes in. Nothing licensed under the GPL family goes into a plugin that VideoCraw distributes. A model or data file is recorded with its licence next to the plugin that uses it.

## Still open

1. A third-party notices file for a distributed plugin (the tables are the raw material).
2. The FFmpeg libraries inside the PyAV wheel (see above) if the speech plugin is ever distributed with its Python environment.
3. The licence of each speech model that is recommended.
