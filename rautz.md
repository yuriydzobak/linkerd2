# Route-oriented policy

Let's start with a few servers:

## HTTP interfaces

Let's use an example HTTP service that exercises a variety of request types:

### API

```text
GET / HTTP/1.1
host: example.com
```

```text
GET /api/params?key=value&label HTTP/1.1
host: example.com
```

### Resources

```yaml
apiVersion: policy.linkerd.io/v1beta1
kind: Server
metadata:
  namespace: eg
  name: example-http
  labels:
    app: example
    protocol: http
spec:
  podSelector:
    matchLabels:
      app: example
  port: http
  proxyProtocol: HTTP/1
```

```yaml
apiVersion: policy.linkerd.io/v1beta1
kind: HTTPInterface
metadata:
  namespace: eg
  name: example-http
spec:
  paths:
    - path: /
      methods:
        - GET
        - HEAD
        - OPTIONS
      labels:
        foo: bar
```

```yaml
apiVersion: policy.linkerd.io/v1beta1
kind: HTTPServerBinding
metadata:
  namespace: eg
  name: example-http
spec:
  interfaceName: example-http
  servers:
    - name: example-http
    #- matchLabels:
    #    app: example
    #    protocol: http
  hosts:
    - name: example.default.svc.cluster.local
    - name: example.com
    - suffix: "*.example.com"
```

## GRPC interfaces

Let's use an example protobuf service that exercises a variety of RPC types:

```proto3
message Request {}

message Response {}

service Example {
  // A simple unary endpoint.
  //
  // - Response timeouts: yes
  // - Retryable: yes
  rpc Ping(Request) returns(Response) {}

  // Streams requests.
  //
  // - Response timeouts: limits time to response message from initial request
  //   message
  // - Retryable: no
  rpc Notify(stream Request) returns(Response) {}

  // Streams responses.
  //
  // - Response timeouts: limits time to first response message.
  // - Retryable: when no response has been received
  rpc Watch(Request) returns(stream Response) {}

  // Streams requests and responses.
  //
  // - Response timeouts: limits time to first response message from initial
  //   request message
  // - Retryable: no
  rpc Duplex(stream Request) returns(stream Response) {}
}
```

```yaml
apiVersion: policy.linkerd.io/v1beta1
kind: Server
metadata:
  namespace: eg
  name: example-grpc
  labels:
    app: example
    protocol: grpc
  podSelector:
    matchLabels:
      app: example
  port: grpc
  proxyProtocol: gRPC
```

```yaml
apiVersion: policy.linkerd.io/v1beta1
kind: GRPCInterface
metadata:
  namespace: eg
  name: example-grpc
spec:
  service: Example
  rpcs:
    - name: Ping
    - name: Notify
      request:
        stream: true
    - name: Watch
      response:
        stream: true
    - name: Duplex
      request:
        stream: true
      response:
        stream: true
```
