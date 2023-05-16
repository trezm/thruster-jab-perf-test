# Counter

### jab-ex
```
Running 30s test @ http://localhost:8080/ping
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   100.10us   79.43us   5.17ms   99.22%
    Req/Sec    43.07k     1.59k   45.12k    88.04%
  2579583 requests in 30.10s, 371.47MB read
Requests/sec:  85701.79
Transfer/sec:     12.34MB
```

### static-ex
```
Running 30s test @ http://localhost:8080/ping
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   108.40us  199.96us   9.68ms   99.67%
    Req/Sec    42.46k     2.85k   62.35k    92.51%
  2539564 requests in 30.10s, 364.65MB read
Requests/sec:  84361.88
Transfer/sec:     12.11MB
```
# ServerConfig

### jab-ex
```
Running 30s test @ http://localhost:8080/name
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   107.03us  190.07us  10.09ms   99.61%
    Req/Sec    42.84k     3.05k   45.85k    90.70%
  2566315 requests in 30.10s, 359.77MB read
Requests/sec:  85260.02
Transfer/sec:     11.95MB
```

### static-ex
```
Running 30s test @ http://localhost:8080/name
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   119.44us  369.74us  15.77ms   99.43%
    Req/Sec    42.66k     4.05k   45.31k    94.35%
  2555259 requests in 30.10s, 358.22MB read
Requests/sec:  84892.15
Transfer/sec:     11.90MB
```
