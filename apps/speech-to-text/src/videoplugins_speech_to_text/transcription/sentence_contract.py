"""The shapes the transcription passes around."""

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class Word:
    """A word as the speech recognition heard it, with its time in seconds."""

    start: float
    end: float
    #: The word as written, usually with the space in front of it.
    text: str
    #: How sure the recognition is of it, from 0 to 1.
    probability: float


@dataclass(frozen=True)
class Sentence:
    """A sentence, with the time it starts and ends in whole milliseconds."""

    start_ms: int
    end_ms: int
    text: str
    #: The average sureness of its words, from 0 to 1.
    confidence: float

    def to_dict(self) -> dict[str, Any]:
        """What the action gives for this sentence."""
        return {
            "startMs": self.start_ms,
            "endMs": self.end_ms,
            "text": self.text,
            "confidence": self.confidence,
        }
