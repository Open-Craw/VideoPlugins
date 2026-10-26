# Licences of what the plugins are built with

Written 2026-10-26. This page records **what the plugins in this repository use and under which licences, as the packages say about themselves**. It is an inventory, not a legal opinion; the questions that need one are at the end.

## What is where

| What | How it is used | Licence situation |
|---|---|---|
| The Rust crates below | linked into the Rust plugins (`sound-volume`, `sound-graph`, `sound-clean`, `thumbnail`, `example-hits`, `coco-objects`, `sound-events`) | permissive, except the **symphonia** crates (MPL-2.0, see the notes) |
| The Python packages below | installed for `speech-to-text` | permissive, except `tqdm` and `certifi` (MPL-2.0) |
| **FFmpeg** | **not used by any plugin.** VideoCraw decodes the media and hands it over | the plugins carry no decoder of their own except what is said below |
| **ONNX Runtime** | linked into `coco-objects` and `sound-events` by the `ort` crate, which fetches the build Microsoft publishes when the plugin is built (nothing extra ships next to the program) | MIT; its own notices list the third-party code in the build: to be read before `coco-objects` is distributed |
| Sound model | **not packaged.** `sound-events` downloads YAMNet (an ONNX conversion of Google's model, 16 MB) from Hugging Face, once, and checks its sha256 | YAMNet is Apache-2.0 (Google); the conversion's page says Apache-2.0 too: to be confirmed against the original before the plugin is distributed |
| Detection models | **not packaged.** `coco-objects` downloads the YOLOX model asked for (nano, tiny or s) from the release of its authors, once, and checks its sha256 | the code and the weights of YOLOX are Apache-2.0, as its repository says |
| Speech models | **not packaged.** `speech-to-text` downloads the model you name (tiny by default) from the Hugging Face hub, once | the licence of each model is read from its model card at the place it is downloaded from; the weights of OpenAI Whisper are published under MIT, and the converted ones used by faster-whisper by the people who converted them: to be confirmed for each model before it is recommended |

## Notes

- **symphonia (MPL-2.0).** The sound reader (`libs/sound-reader`) uses it to read mp3, wav, flac, ogg, aac and alac files when a plugin is given a file and not the sound VideoCraw decoded. MPL-2.0 is a file-level copyleft: using the crates unmodified, in a program of another licence, is allowed; the notice and the source of *those files* (not of the plugin) have to be available to whoever gets the program. The plugins do not modify them.
- **ring, rustls and the root certificates (webpki).** `coco-objects` and `sound-events` download their model (through `libs/model-cache`) with `ureq`, which brings `ring` (Apache-2.0 AND ISC; it carries code derived from BoringSSL and OpenSSL, whose notices go with it), `rustls` and the certificate list of `webpki-roots` (CDLA-Permissive-2.0). The `openssl` crates in the list come only from the build script of `ort-sys` (it downloads ONNX Runtime when the plugin is built); a build dependency is not linked into the program.
- **av (PyAV, BSD-3-Clause).** `faster-whisper` depends on it to decode audio when it is given a file. Its binary wheels carry FFmpeg libraries, whose own licence (LGPL, or GPL if built with GPL parts) is separate from the BSD licence of the Python wrapper: to be checked for the wheel that is installed before the plugin is distributed. When VideoCraw hands the plugin decoded samples, none of it is used.
- **tqdm and certifi (MPL-2.0).** Same file-level terms as above; used unmodified.
- **Build tools** (ruff, mypy, pytest, and so on) are not part of what a plugin ships and are left out of the lists.

## The packages

Regenerate with `python tools/list_licences.py <this repository> --python <its python> faster-whisper numpy` from the VideoCraw repository (the script is there); it reads what each package says and does not read licence texts.

### Rust crates

- 75 x MIT OR Apache-2.0
- 13 x MPL-2.0
- 10 x Apache-2.0 OR MIT
- 9 x MIT/Apache-2.0
- 9 x MIT
- 3 x ISC
- 3 x Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT
- 2 x Apache-2.0
- 2 x MIT OR Zlib OR Apache-2.0
- 2 x CDLA-Permissive-2.0
- 1 x 0BSD OR MIT OR Apache-2.0
- 1 x Zlib OR Apache-2.0 OR MIT
- 1 x (Apache-2.0 OR MIT) AND BSD-3-Clause
- 1 x Unlicense OR MIT
- 1 x MIT OR Apache-2.0 OR LGPL-2.1-or-later
- 1 x Apache-2.0 AND ISC
- 1 x Apache-2.0 OR ISC OR MIT
- 1 x BSD-3-Clause
- 1 x (MIT OR Apache-2.0) AND Unicode-3.0
- 1 x Zlib

| Package | Version | Licence |
|---|---|---|
| adler2 | 2.0.1 | 0BSD OR MIT OR Apache-2.0 |
| arrayvec | 0.7.8 | MIT OR Apache-2.0 |
| autocfg | 1.5.1 | Apache-2.0 OR MIT |
| base64 | 0.23.1 | MIT OR Apache-2.0 |
| base64ct | 1.8.3 | Apache-2.0 OR MIT |
| bitflags | 1.3.2 | MIT/Apache-2.0 |
| bitflags | 2.13.2 | MIT OR Apache-2.0 |
| block-buffer | 0.10.4 | MIT OR Apache-2.0 |
| bytemuck | 1.25.2 | Zlib OR Apache-2.0 OR MIT |
| bytes | 1.12.1 | MIT |
| cc | 1.10.0 | MIT OR Apache-2.0 |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 |
| core-foundation | 0.10.1 | MIT OR Apache-2.0 |
| core-foundation-sys | 0.8.7 | MIT OR Apache-2.0 |
| core_detect | 1.0.0 | MIT/Apache-2.0 |
| cpufeatures | 0.2.17 | MIT OR Apache-2.0 |
| crc32fast | 1.5.3 | MIT OR Apache-2.0 |
| crypto-common | 0.1.7 | MIT OR Apache-2.0 |
| der | 0.8.3 | Apache-2.0 OR MIT |
| digest | 0.10.7 | MIT OR Apache-2.0 |
| encoding_rs | 0.8.42 | (Apache-2.0 OR MIT) AND BSD-3-Clause |
| errno | 0.3.14 | MIT OR Apache-2.0 |
| extended | 0.1.0 | MIT |
| fastrand | 2.5.0 | Apache-2.0 OR MIT |
| fdeflate | 0.3.7 | MIT OR Apache-2.0 |
| find-msvc-tools | 0.1.15 | MIT OR Apache-2.0 |
| flate2 | 1.1.10 | MIT OR Apache-2.0 |
| foreign-types | 0.3.2 | MIT/Apache-2.0 |
| foreign-types-shared | 0.1.1 | MIT/Apache-2.0 |
| generic-array | 0.14.7 | MIT |
| getrandom | 0.2.17 | MIT OR Apache-2.0 |
| getrandom | 0.4.3 | MIT OR Apache-2.0 |
| hmac-sha256 | 1.1.15 | ISC |
| http | 1.5.0 | MIT OR Apache-2.0 |
| httparse | 1.10.1 | MIT OR Apache-2.0 |
| itoa | 1.0.18 | MIT OR Apache-2.0 |
| lazy_static | 1.5.1 | MIT OR Apache-2.0 |
| libc | 0.2.191 | MIT OR Apache-2.0 |
| linux-raw-sys | 0.12.1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT |
| log | 0.4.34 | MIT OR Apache-2.0 |
| lzma-rust2 | 0.15.8 | Apache-2.0 |
| matrixmultiply | 0.3.11 | MIT/Apache-2.0 |
| memchr | 2.8.3 | Unlicense OR MIT |
| memmap2 | 0.9.12 | MIT OR Apache-2.0 |
| miniz_oxide | 0.8.9 | MIT OR Zlib OR Apache-2.0 |
| miniz_oxide | 0.9.1 | MIT OR Zlib OR Apache-2.0 |
| multiversion_no_op | 1.0.0 | Apache-2.0 OR MIT |
| native-tls | 0.2.18 | MIT OR Apache-2.0 |
| ndarray | 0.17.2 | MIT OR Apache-2.0 |
| num-complex | 0.4.6 | MIT OR Apache-2.0 |
| num-integer | 0.1.47 | MIT OR Apache-2.0 |
| num-traits | 0.2.19 | MIT OR Apache-2.0 |
| once_cell | 1.21.4 | MIT OR Apache-2.0 |
| openssl | 0.10.81 | Apache-2.0 |
| openssl-macros | 0.1.1 | MIT/Apache-2.0 |
| openssl-probe | 0.2.2 | MIT OR Apache-2.0 |
| openssl-sys | 0.9.117 | MIT |
| ort | 2.0.0-rc.15 | MIT OR Apache-2.0 |
| ort-sys | 2.0.0-rc.15 | MIT OR Apache-2.0 |
| pem-rfc7468 | 1.0.0 | Apache-2.0 OR MIT |
| percent-encoding | 2.3.2 | MIT OR Apache-2.0 |
| pin-project-lite | 0.2.17 | Apache-2.0 OR MIT |
| pkg-config | 0.3.34 | MIT OR Apache-2.0 |
| png | 0.18.1 | MIT OR Apache-2.0 |
| portable-atomic | 1.15.0 | Apache-2.0 OR MIT |
| portable-atomic-util | 0.2.8 | Apache-2.0 OR MIT |
| primal-check | 0.3.4 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| r-efi | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later |
| rawpointer | 0.2.1 | MIT/Apache-2.0 |
| ring | 0.17.14 | Apache-2.0 AND ISC |
| rustfft | 6.4.1 | MIT OR Apache-2.0 |
| rustix | 1.1.5 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT |
| rustls | 0.23.46 | Apache-2.0 OR ISC OR MIT |
| rustls-pki-types | 1.15.1 | MIT OR Apache-2.0 |
| rustls-webpki | 0.103.16 | ISC |
| rustversion | 1.0.23 | MIT OR Apache-2.0 |
| schannel | 0.1.30 | MIT |
| scopeguard | 1.2.0 | MIT OR Apache-2.0 |
| security-framework | 3.7.0 | MIT OR Apache-2.0 |
| security-framework-sys | 2.17.0 | MIT OR Apache-2.0 |
| serde | 1.0.229 | MIT OR Apache-2.0 |
| serde_core | 1.0.229 | MIT OR Apache-2.0 |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 |
| sha2 | 0.10.9 | MIT OR Apache-2.0 |
| shlex | 2.0.1 | MIT OR Apache-2.0 |
| simd-adler32 | 0.3.10 | MIT |
| simdutf8 | 0.1.5 | MIT OR Apache-2.0 |
| smallvec | 1.16.4 | MIT OR Apache-2.0 |
| strength_reduce | 0.2.4 | MIT OR Apache-2.0 |
| subtle | 2.6.1 | BSD-3-Clause |
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
| syn | 2.0.119 | MIT OR Apache-2.0 |
| syn | 3.0.9 | MIT OR Apache-2.0 |
| tempfile | 3.27.0 | MIT OR Apache-2.0 |
| tracing | 0.1.44 | MIT |
| tracing-core | 0.1.36 | MIT |
| transpose | 0.2.3 | MIT OR Apache-2.0 |
| typenum | 1.20.1 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| untrusted | 0.9.0 | ISC |
| ureq | 3.5.0 | MIT OR Apache-2.0 |
| ureq-proto | 0.6.5 | MIT OR Apache-2.0 |
| utf8-zero | 0.8.1 | MIT OR Apache-2.0 |
| vcpkg | 0.2.15 | MIT/Apache-2.0 |
| version_check | 0.9.5 | MIT/Apache-2.0 |
| wasi | 0.11.1+wasi-snapshot-preview1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT |
| webpki-root-certs | 1.0.11 | CDLA-Permissive-2.0 |
| webpki-roots | 1.0.11 | CDLA-Permissive-2.0 |
| windows-link | 0.2.1 | MIT OR Apache-2.0 |
| windows-sys | 0.52.0 | MIT OR Apache-2.0 |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 |
| windows-targets | 0.52.6 | MIT OR Apache-2.0 |
| windows_aarch64_gnullvm | 0.52.6 | MIT OR Apache-2.0 |
| windows_aarch64_msvc | 0.52.6 | MIT OR Apache-2.0 |
| windows_i686_gnu | 0.52.6 | MIT OR Apache-2.0 |
| windows_i686_gnullvm | 0.52.6 | MIT OR Apache-2.0 |
| windows_i686_msvc | 0.52.6 | MIT OR Apache-2.0 |
| windows_x86_64_gnu | 0.52.6 | MIT OR Apache-2.0 |
| windows_x86_64_gnullvm | 0.52.6 | MIT OR Apache-2.0 |
| windows_x86_64_msvc | 0.52.6 | MIT OR Apache-2.0 |
| zeroize | 1.9.1 | Apache-2.0 OR MIT |
| zlib-rs | 0.6.8 | Zlib |
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
