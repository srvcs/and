# srvcs-and

A logic primitive of the srvcs.cloud distributed standard library.

Its single concern: **the boolean AND of two operands.** It is a *leaf* — it
depends on no other service and validates its own input. Given two JSON
booleans `a` and `b`, it returns `a && b`.

## API

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/` | Service identity, concern, and dependency list |
| `POST` | `/` | Evaluate `a && b` |
| `GET` | `/healthz` `/readyz` `/metrics` `/openapi.json` | srvcs service standard surface |

```sh
curl -s -X POST localhost:8080/ -H 'content-type: application/json' -d '{"a": true, "b": true}'
# {"a":true,"b":true,"result":true}

curl -s -X POST localhost:8080/ -H 'content-type: application/json' -d '{"a": true, "b": false}'
# {"a":true,"b":false,"result":false}

curl -s -X POST localhost:8080/ -H 'content-type: application/json' -d '{"a": 1, "b": true}'
# 422 {"error":"a is not a boolean"}
```

`POST /` requires both `a` and `b` to be JSON booleans. Any non-boolean operand
(number, string, `null`, array, or object) is rejected with `422`.

## Dependencies

None. `srvcs-and` is a leaf.

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `SRVCS_BIND_ADDR` | `0.0.0.0:8080` | Bind address |
| `SRVCS_ENV` | `development` | Environment label for logs |
| `RUST_LOG` | `info,tower_http=info` | Tracing filter |

## Local checks

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The full Nix gates and OCI image build are documented in
[`srvcs/platform`](https://github.com/srvcs/platform); CI runs them through the
shared `build-service.yml` workflow.

> Note: the `cargoHash` in `flake.nix` is inherited from the template and must be
> refreshed with a `nix build` before the Nix gates pass.
