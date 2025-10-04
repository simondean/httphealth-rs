# httphealth

A binary for performing HTTP based health checks in a Docker container

Example usage:

```shell
./healthcheck http://0.0.0.0:8000 200
```

In the example above, 200 is the expected status code.
