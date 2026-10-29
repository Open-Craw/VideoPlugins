# music-sync

Finds where a music starts inside a video's sound, and how much faster or slower the video plays it. A VideoCraw plugin written in Rust, so it is one program with nothing to install.

The use it was made for: a video has its music in low quality, you have the same music in good quality. This plugin says how late the good copy has to start to line up with the video; `media.replaceAudio` (with its `offsetMs`, `speed` and `keep`) then makes a new video with the original pictures and the good music.

## Action `findOffset`

| Input | |
|---|---|
| `video` | the video (or any sound) that carries the music in low quality |
| `music` | the music in good quality: a sound file or a video |
| `minScoreSigma` | how far the best match must stand out from chance, 2 to 100 (default 30). Below it the call fails with 422, so a wrong music is never put in a video |
| `minOverlapMs` | the least the two sounds must share, 500 to 1800000 (default 5000) |

It gives:

| Output | |
|---|---|
| `offsetMs` | how late the music starts on the video's timeline. Negative when the video starts partway into the music |
| `score` | how far the match stands out from chance, in standard deviations |
| `overlapMs` | how much of the music lies inside the video |
| `speed` | how much longer the video takes than the music for the same stretch: 1 for the same speed, 1.05 when the video plays the music 5% slower |
| `piecesOnLine` | how many pieces of the music agreed on that speed; `null` when too few were found to tell |

```json
{
  "Where": { "type": "music-sync.findOffset", "inputs": { "video": "@body('Open')", "music": "C:/music/song.flac" } },
  "Better": { "type": "media.replaceAudio", "runAfter": { "Where": ["Succeeded"] },
    "inputs": { "video": "@body('Open')", "sound": "C:/music/song.flac", "offsetMs": "@body('Where').offsetMs", "speed": "@body('Where').speed", "keep": "video" } }
}
```

The music goes into the video from the original file at full quality; this plugin only listens, at 8 kHz mono, to find the number.

### Many videos and many musics

The flow runs once for each video. To find the music that goes with each one, list the music folder and match by file name (the stem is the name without the extension):

```json
"Musics": { "type": "data.listFiles", "inputs": { "from": ["music"], "extensions": ["flac", "mp3"] } },
"Same": { "type": "data.where", "runAfter": { "Musics": ["Succeeded"] },
  "inputs": { "items": "@body('Musics')", "field": "stem", "op": "equals", "value": "@triggerBody().stem" } },
"One": { "type": "data.pick", "runAfter": { "Same": ["Succeeded"] },
  "inputs": { "items": "@body('Same')", "which": "first", "required": true, "message": "@concat('no music named ', triggerBody().stem)" } },
```

then give `@body('One').path` to `findOffset` and `replaceAudio`. A video with no music of its name fails with that message and the others carry on. Any other field can be matched the same way (`name`, `extension`, `mime`, `directory`).

## Keeping the video's speed or the music's

When `speed` is not 1 the two cannot both keep their speed. `media.replaceAudio` takes `speed` and `keep`:

- `keep: "video"` (the default): the picture is untouched and the music is stretched to fit it (pitch kept, FFmpeg `atempo`).
- `keep: "audio"`: the music is untouched and the picture is sped up or slowed down to fit it. The picture is made again (H.264, CRF 18), so it takes longer.

## How it works

Both sounds are decoded by VideoCraw to 8 kHz mono and compared with phase correlation (every frequency counts the same, only its timing matters), which survives a copy with a different tone, volume or noise.

1. The best delay over the whole length, and a line through ten 10-second pieces of the music found in the video, which gives the speed when it is a little off 1.
2. When no line confirms the speed (the video runs at another speed, so the whole-length match is smeared), speeds from 0.85 to 1.25 in steps of 0.001 are tried: five pieces of the music are stretched by each and looked for in the video at a quarter of the rate, and the speed where most pieces agree on the start wins.
3. The music is stretched by the speed found and the speed is narrowed by golden-section search on the strength of the match over the whole length.

## What was measured

On a made-up 40 s music (synthetic notes and a beat; no real song was used), the same music through a 3.5 kHz low-pass, half the volume and a 24 kbps AAC, put in a video:

| Case | `offsetMs` | `speed` | `score` |
|---|---|---|---|
| 3.3 s late, same speed | 3300 | 1 | 1984 |
| the video starts 5 s into the music | -5000 | 1 | 415 |
| 2 s late and 5% slower (true values 2000 and 1.05) | 2000 | 1.049997 | 1865 |
| a different synthetic music | refused, 422 | | 14 |

With the 5% slower video, `keep: "video"` gave a video of the original length with the music stretched to fit it, and `keep: "audio"` gave a video 1.05 times shorter with the music at its own speed (durations checked with ffprobe; not listened to).

## Limits

- One speed for the whole video. A video that cuts the music and joins it again is not handled.
- The speed search is slow: it took 13 s on this 40 s video and grows with the video's length, so a 4-minute video may take a minute or two. It only runs when the plain match is not confirmed.
- The default `minScoreSigma` 30 and the speed search were set from these few synthetic cases, not from a collection of real songs. The first versions of the speed search failed on the degraded copy (steps of 0.003 and the beat of the music did not find it), so a heavily degraded real copy at an odd speed may still be missed: check the result by listening.
- Sounds up to 30 minutes each; it needs memory in proportion (about 1.5 GB at the limit).
- Speech or effects over the music lower the score but rarely move the offset.

## Licences

Rust crates only (`rustfft`, MIT or Apache-2.0, and the shared sound reader), see `docs/licences.md`.
