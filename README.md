# Feature Estimator Assistant

A CLI utility for estimating delivery time of a feature based on the description of the feature
using a model based on the paper [Unified Time Scaling for Temporal Coordination
Frameworks](https://www.rfc-editor.org/rfc/rfc9759.txt) 

## Installation

Install this tool using `cargo`:
```bash
cargo install path . --root ~/.local/
```

## Getting started

Using the model is fairly simple. The tool will prompt you to provide a description of the feature
along with a link to the source code for the tool to analyze. From there the tool will calculate an
estimate.

