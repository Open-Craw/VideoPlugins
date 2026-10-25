"""Starts the plugin: `python -m videoplugins_speech_to_text`. VideoCraw does this itself."""

from .actions import run_action
from .serve import serve_stdio

serve_stdio(run_action)
