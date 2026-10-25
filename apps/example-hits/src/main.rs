//! A reference VideoCraw plugin. It pretends to find things in a video, so recipes and hosts can be
//! tried and tested without a model. See docs/plugin-protocol.md for the protocol it speaks.

mod methods;
mod serve;

fn main() {
    let input = std::io::stdin();
    let output = std::io::stdout();
    serve::serve_lines(input.lock(), output.lock());
}
