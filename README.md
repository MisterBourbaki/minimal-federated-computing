# A minimal(ist) Federated computation framework

## Why?

This crate is just (at least for now) a playtest of what could be a minimal federated computation framework. The goal is to use Tonic framework, from the Rust ecosystem, with the Pyo3 crate for Python/Rust bindings to get a gRPC framework able to gather Python computations from several "clients".

This is based initially on some examples from Tonic and Rust Numpy crates.

## Use

To run the code, first install numpy in a dedicated virtual env.

```bash
python -m pip install virtualenv
python -m virtualenv .venv
source .venv/bin/activate
```

Please note that the python interpreter and the virtual env have to be findable by PyO3.

Then run 

```bash
(.venv) cargo run --bin mfc-server
```

to launch the server, and in another termianl window

```bash
(.venv) cargo run --bin mfc-client
```