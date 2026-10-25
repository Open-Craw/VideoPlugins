"""Listens to a video or a sound and writes down the sentences, with their times."""

import threading
from collections.abc import Callable
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from ..serve import Failure
from .sentence_contract import Word
from .speech_model_store import get_model
from .split_sentences_algorithm import split_sentences


@dataclass(frozen=True)
class TranscribeOptions:
    #: The model: `tiny`, `base`, `small`, `medium`, `large-v3`, with `.en` for English only.
    model: str = "tiny"
    #: The language as a code such as `en`; `None` to find it out from the first seconds.
    language: str | None = None
    #: Skip the stretches with no speech. It stops the model from inventing words in silence, but
    #: it also drops speech that sits over loud music, so it is off unless asked for.
    skip_silence: bool = False
    beam_size: int = 5


def transcribe_media(
    media: Path,
    options: TranscribeOptions,
    cancel: threading.Event,
    load_model: Callable[[str], Any] = get_model,
) -> dict[str, Any]:
    """Gives `{ language, languageProbability, durationMs, model, text, sentences }`.

    `sentences` is a list of `{ startMs, endMs, text, confidence }`. A video or sound with no speech
    in it is a success with no sentences. `load_model` is how a model is got by its name (the
    recognition itself can be swapped in tests).
    """
    try:
        model = load_model(options.model)
    except Exception as error:
        # Usually no network the first time, or a name that is not a model.
        raise Failure.status(
            503, f"the model {options.model!r} could not be loaded: {error}"
        ) from error

    words: list[Word] = []
    try:
        segments, info = model.transcribe(
            str(media),
            language=options.language,
            beam_size=options.beam_size,
            word_timestamps=True,
            vad_filter=options.skip_silence,
        )
        for segment in segments:  # the recognition runs as this is read
            if cancel.is_set():
                raise Failure("cancelled", code="CANCELLED")
            for word in segment.words or []:
                words.append(Word(word.start, word.end, word.word, word.probability))
    except Failure:
        raise
    except Exception as error:
        raise Failure.status(422, f"cannot read speech from {media.name}: {error}") from error

    sentences = split_sentences(words)
    return {
        "language": info.language,
        "languageProbability": round(float(info.language_probability), 3),
        "durationMs": round(float(info.duration) * 1000),
        "model": options.model,
        "text": " ".join(sentence.text for sentence in sentences),
        "sentences": [sentence.to_dict() for sentence in sentences],
    }
