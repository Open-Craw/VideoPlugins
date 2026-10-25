"""The protocol over standard input and output: one JSON message per line in, one per line out."""

from .failure_error import Failure
from .serve_lines_handler import Handler, serve_lines, serve_stdio

__all__ = ["Failure", "Handler", "serve_lines", "serve_stdio"]
