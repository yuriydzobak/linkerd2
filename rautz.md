# Route-oriented policy

Let's start with a few servers:

```yaml
apiVersion: policy.linkerd.io/v1beta1
kind: Server
metadata:
  namespace: emojivoto
  name: emoji-grpc
  labels:
    emojivoto/api: internal-grpc
spec:
  podSelector:
    matchLabels:
      app: emoji-svc
  port: grpc
  proxyProtocol: gRPC

---
# Server "web-http": matches the http port for pods in the web service, by
# selecting over the app=web-svc label.
apiVersion: policy.linkerd.io/v1beta1
kind: Server
metadata:
  namespace: emojivoto
  name: web-http
spec:
  podSelector:
    matchLabels:
      app: web-svc
  port: http
  proxyProtocol: HTTP/1
```

```yaml
---
# Server "web-http": matches the http port for pods in the web service, by
# selecting over the app=web-svc label.
apiVersion: policy.linkerd.io/v1beta1
kind: GRPCInterface
metadata:
  namespace: emojivoto
  name: emoji-grpc
spec:
  service: EmojiService
  rpcs:
    - name: ListAll
    - name: FindByShortcode
```

---

```yaml
---
# ServerAuthorization "web-public": allows unauthenticated traffic
# to the web-http Server, so that the web service can serve HTTP requests
# to anyone.
apiVersion: policy.linkerd.io/v1beta1
kind: ServerAuthorization
metadata:
  namespace: emojivoto
  name: web-public
  labels:
    app.kubernetes.io/part-of: emojivoto
    app.kubernetes.io/name: web
    app.kubernetes.io/version: v11
spec:
  server:
    name: web-http
  client:
    unauthenticated: true
    networks:
      - cidr: 0.0.0.0/0
      - cidr: ::/0

---
# Server "prom": matches the Prometheus port of the emoji, web, and voting
# services, by selecting over the pods with corresponding app labels.
apiVersion: policy.linkerd.io/v1beta1
kind: Server
metadata:
  namespace: emojivoto
  name: prom
  labels:
    app.kubernetes.io/part-of: emojivoto
    app.kubernetes.io/version: v11
spec:
  port: prom
  podSelector:
    matchExpressions:
      - key: app
        operator: In
        values: [emoji-svc, web-svc, voting-svc]
  proxyProtocol: HTTP/1

---
# ServerAuthorization "prom-prometheus": allows unauthenticated traffic to the
# "prom" Server, so that metrics scrapes can come from anywhere.
apiVersion: policy.linkerd.io/v1beta1
kind: ServerAuthorization
metadata:
  namespace: emojivoto
  name: prom-prometheus
  labels:
    app.kubernetes.io/part-of: emojivoto
    app.kubernetes.io/version: v11
spec:
  server:
    name: prom
  client:
    unauthenticated: true

---
# Server "admin": matches the admin port for every pod in this namespace
apiVersion: policy.linkerd.io/v1beta1
kind: Server
metadata:
  namespace: emojivoto
  name: admin
  labels:
    app.kubernetes.io/part-of: emojivoto
    app.kubernetes.io/version: v11
spec:
  port: linkerd-admin
  podSelector:
    matchLabels: {} # every pod
  proxyProtocol: HTTP/1

---
# ServerAuthorization "admin-everyone": allows unauthenticated access to the
# "admin" Server, so that Kubernetes health checks can get through.
apiVersion: policy.linkerd.io/v1beta1
kind: ServerAuthorization
metadata:
  namespace: emojivoto
  name: admin-everyone
  labels:
    app.kubernetes.io/part-of: emojivoto
    app.kubernetes.io/version: v11
spec:
  server:
    name: admin
  client:
    unauthenticated: true
```
