"""Runs an action of the plugin."""

import threading
from typing import Any

from ..serve import Failure
from ..transcription import transcribe_media
from .read_params_mapper import read_media, read_options


def run_action(action: str, params: dict[str, Any], cancel: threading.Event) -> dict[str, Any]:
    """Runs `transcribe`: the sentences said in a video or a sound, with their times.

    Params: `media` (a video or a sound), `model` (default `tiny`), `language` (default: found out),
    `skipSilence` (default false) and `beamSize` (default 5).
    """
    if action == "transcribe":
        return transcribe_media(read_media(params), read_options(params), cancel)
    raise Failure.status(404, f"this plugin has no action {action!r} (it has transcribe)")
