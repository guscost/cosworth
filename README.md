# Cosworth

A Rust framework for building JSON web services

## Purpose

This is a tool for building web services to be compiled into Rust static binaries and deployed into Docker containers. It is very much a work in progress, so there is no tutorial yet.

It should be useful in production environments where safety and *predictable* performance take precedence over built-in frontends, formats other than JSON, full compliance with [JSON API](http://jsonapi.org), streaming responses, etc. The production server should compile into a Docker image that weighs less than 25MB and responds faster than anything written in a memory-managed language (in the worst case).

## Goals

Web services built with this tool have the following goals, in approximate order of importance:

1. **Safe:** The server should be difficult to exploit or DoS.
2. **Predictable:** The server should not slow down during operation.
3. **Efficient:** The server should not waste CPU, memory, or disk space.
4. **Fast:** The server should handle a lot of requests quickly.
6. **Simple:** The server should not be more complex than is necessary.
5. **Easy:** The server should not be difficult to develop or debug.

While some other frameworks run faster and most will be easier to get started using, they also have one or more disadvantages:

- Garbage-collected languages run in a turing-complete VM, not on the actual hardware. This requires much more memory than unmanaged code, and makes it difficult or impossible to prevent unpredictable variation in performance.
- Simpler unmanaged languages like C (and to some degree, C++) do not enforce safe memory access. It is possible to build safe services using these languages, but deep experience and additional tools (e.g. Valgrind) are required.
- Many web frameworks use dynamically-typed languages, leading to a culture that worships test-driven development and elaborate runtime debuggers.
- Most web frameworks try to accommodate as many use cases as possible, which tends to make them either "too big" or "too little".
- Some bake in support for lots of "batteries included" features. Often these frameworks are more mature, and were originally designed to serve complete web applications including frontends. When these extra features are not needed they can often get in the way.
- Others ("micro-frameworks") only ship with an extensible core around which to build, so that they can be combined with features from many other projects. This prevents them from optimizing for or officially supporting non-trivial architecture, which means developers usually have to hunt down and validate lots of dependencies.

By being both prescriptive and modest about the scope from the start, the hope is that this framework will maintain a balance between the extremes which is best suited for achieving its goals.

## Roadmap

Support is planned for these features:

- Asynchronous request handling
- Postgres with connection pool and ORM
- Signed bearer token authentication
- Authorization with User and Token models
- Authorization with Redis lookup
- Authorization with proxy HTTP request
- REST resource validation and serialization
- Generic endpoints or "views" (http://www.django-rest-framework.org/api-guide/generic-views/)

Support is under consideration for these features:

- GraphQL
- WebSocket
- Redis pub/sub
- Pluggable OAuth
- Roles and user management

Current dependencies include [diesel](http://diesel.rs) for accessing databases, [serde](http://serde.js) for manipulating JSON, and [actix-web](http://actix.rs) for serving HTTP requests. The architecture will probably use the [actix](https://github.com/actix/actix) actor system underneath actix-web.
