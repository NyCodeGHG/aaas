# Asciinema as a Service

[![CI](https://github.com/NyCodeGHG/aaas/actions/workflows/ci.yaml/badge.svg)](https://github.com/NyCodeGHG/aaas/actions/workflows/ci.yaml)
[![GitHub License](https://img.shields.io/github/license/NyCodeGHG/stellwerksim-rich-presence?style=flat-square)](LICENSE)

Small microservice for rendering Asciicasts to gifs.

## Configuration

### OpenTelemetry

aaas supports sending traces to a an observability backend via the [OTLP Protocol](https://opentelemetry.io/docs/specs/otlp/).

| Environment Variable          | Description                                                                                                   |
|-------------------------------|---------------------------------------------------------------------------------------------------------------|
| `ENABLE_OTLP`                 | Enables OpenTelemetry trace export                                                                            |
| `OTEL_SERVICE_NAME`           | Sets the Service Name                                                                                         |
| `OTEL_EXPORTER_OTLP_PROTOCOL` | The OTLP protocol to use. `grpc` or `http/protobuf`. Defaults to `grpc`                                       |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | The OTLP endpoint to use. Defaults to `http://localhost:4317` with gRPC or `http://localhost:4318` with http. |
| `OTEL_EXPORTER_OTLP_TIMEOUT`  | The timeout to use for the trace export. Defaults to 10.                                                      |

## Endpoints

aaas provides a single endpoint at `/render`.
It expects an asciicast file in binary as the request body and will respond with a 200 `image/gif` response if the image could be successfully rendered.

## Hosting
### Container
A container "docker" image is available [here](https://github.com/NyCodeGHG/aaas/pkgs/container/aaas) on the GitHub container registry.

### Nix
You can also build the package using our nix flake.

```shell
nix build github:NyCodeGHG/aaas
```

## License
aaas is licensed under the [GNU General Public License 3](LICENSE) or https://www.gnu.org/licenses/gpl-3.0.en.html
