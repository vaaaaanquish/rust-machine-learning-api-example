# rust-machine-learning-api-example
Example of Rust API for Machine Learning

API example that uses `resnet224` to infer images received in base64 and returns the results.

Used pytorch rust bindings.

fork (TensorFlow): https://github.com/dskkato/rust-machine-learning-api-example

# Usage

```rust
docker compose up -d --build
python sample_request.py
```
