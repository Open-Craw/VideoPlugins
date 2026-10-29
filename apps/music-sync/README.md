# music-sync

Finds where a music starts inside a video's sound. A VideoCraw plugin written in Rust, so it is one program with nothing to install.

The use it was made for: a video has its music in low quality, you have the same music in good quality. This plugin says how late the good copy has to start to line up with the video; `media.replaceAudio` (with its `offsetMs`) then makes a new video with the original pictures and the good music.

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
| `driftMs` | how much the offset changes from the start to the end of what they share: about 0 for a copy at the same speed. A large number means the video was sped up or slowed down, and one offset cannot line it up everywhere. `null` when the shared stretch is too short to tell |

```json
{
  "Where": { "type": "music-sync.findOffset", "inputs": { "video": "@body('Open')", "music": "C:/music/song.flac" } },
  "Better": { "type": "media.replaceAudio", "runAfter": { "Where": ["Succeeded"] },
    "inputs": { "video": "@body('Open')", "sound": "C:/music/song.flac", "offsetMs": "@body('Where').offsetMs" } }
}
```

The music goes into the video from the original file at full quality; this plugin only listens, at 8 kHz mono, to find the number.

## How it works

Both sounds are decoded by VideoCraw to 8 kHz mono and compared with phase correlation (every frequency counts the same, only its timing matters), which survives a copy with a different tone, volume or noise. The delay with the highest match wins; `score` says how far it stands out from the matches that chance alone gives. Then the two ends of the shared stretch are compared again on their own to measure `driftMs`.

## What was measured

On a made-up 40 s music (synthetic notes and a beat; no real song was used), the same music through a 3.5 kHz low-pass, half the volume and a 24 kbps AAC, put 3.3 s late in a video:

| Case | `offsetMs` | `score` | `driftMs` |
|---|---|---|---|
| music 3.3 s late in the video | 3300 | 1984 | 0 |
| the video starts 5 s into the music | -5000 | 415 | 0 |
| the new video checked against the music again | 3300 (the video's own timeline) | 2813 | 0 |
| a different synthetic music | refused, 422 | 14 | |

The unit tests also cover a music buried in noise, silence, and a video 1% slower (about 180 ms of drift over 20 s).

## Limits

- One offset for the whole video. A video that was sped up or slowed down (`driftMs` far from 0), or one that cuts the music and joins it again, gets the offset of the whole and is out of step elsewhere. Correcting speed is not done.
- The unrelated pair above scored 14, so the default 30 sits between chance and real matches. It was set from these few synthetic cases, not from a collection of real songs: if a real, heavily degraded copy is refused, lower `minScoreSigma` and check the result by listening.
- Sounds up to 30 minutes each; it needs memory in proportion (about 1.5 GB at the limit).
- Only the sound of the video is used; speech or effects over the music lower the score but rarely move the offset.

## Licences

Rust crates only (`rustfft`, MIT or Apache-2.0, and the shared sound reader), see `docs/licences.md`.
