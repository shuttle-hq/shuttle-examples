# HTTP Connect Proxy

Rama's primary aim is to help you make any kind of proxy,
even though it can also just as well be used to create web services or clients.

This example shows how to build a pretty minimal HTTP proxy:

- TLS traffic is not MITM'd in this example;
- We do make use of Basic authentication for the proxy,
- and do so using custom Username labels for optional configuration;
  - such labels are typically used to allow the user to customise certain behaviour on the proxy;
- Finally this example also shows how you can serve a pseudo domain for tunneled traffic only;

Once you run this example using `shuttle run` you'll be able to try the following things:

- use the http proxy to visit a non-encrypted website (`https`), making use of our custom username labels;
```
curl -v -x http://john-priority-high:secret@127.0.0.1:8000 http://example.com
```
- use the http proxy to visit an encrypted website (`https`):
```
curl -v -x http://john:secret@127.0.0.1:8000 https://example.com
```
- use the http proxy to visit our pseudo-domain:
```
curl -x http://john:secret@127.0.0.1:8000 -XPOST http://example.shuttle.local/lucky/42
# {"lucky_number":42}
```
