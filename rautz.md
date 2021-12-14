# Route-oriented policy

## Goals

* Support per-route polices on servers:
  * Authorization
  * Timeouts
  * Add header
  * Remove header
* Support per-route polices on clients:
  * Retries
  * Timeouts
  * Add header
  * Remove header
* Replace `ServiceProfile`
  * Decouple server and client policies
  * Support inbound policies for `Server`s (so that servers may provide a
    canonical route reference).
  * Let service owners publish an interface that can be used by clients to
    configure client-specific policies.

## Proposal

We introduce a new concept to proxies: traffic **labeling**. As each connection
or request is processed by the proxy, it applies a series of labeling rules to
produce a map of key-val labels. Labels are similar to kubernetes `ObjectMeta`
labels.

As the proxy applies policies--authorization, timeouts, retries, header
modification, etc--these policies are applied by selecting over
connection/request labels.

Each connection/request processed by a client or server ***should*** have a
uniform set of labels.

* Should there be a standard set of labels that are always applied for a given
  resource (e.g. `tcp/client-ip`, `http/method`, `grpc/service`)

### Use cases

#### Authorization

#### Timeouts

#### Retries

#### Metrics

* Omitting/rewriting high-cardinality labels

### Performance considerations

* Is it necessary to allocate a new set of labels for each event?
  * Vs hash lookup times to avoid allocation?
* Can label selection be amortized?
  * Easier if all possible label values are known ahead of time?

---

```yaml
apiVersion: rautz.l5d.io/v1alpha1
kind: HTTPLabeler
metadata:
  name: web
  namespace: emojivoto
spec:
  labelers:
    - kind: HttpMethod
      values: ["GET", "HEAD", "OPTIONS", "DELETE", "PUT"]
      labels:
        idempotent: "true"
    - kind: HttpHeader
      header: "l5d-dst-canonical"
      exists: true
```
