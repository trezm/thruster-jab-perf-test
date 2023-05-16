# Multi threaded

static-ex 10s 2 connections

Running 10s test @ http://localhost:8080/ping
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    11.86ms   23.07ms 150.60ms   87.48%
    Req/Sec     3.60k     1.55k    7.41k    68.88%
  71461 requests in 10.07s, 18.95MB read
Requests/sec:   7097.34
Transfer/sec:      1.88MB

jab-ex 10s 2 connections

Running 10s test @ http://localhost:8080/ping
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.23ms   27.70ms 146.09ms   85.79%
    Req/Sec     3.52k     1.52k    6.63k    71.79%
  69990 requests in 10.09s, 18.56MB read
Requests/sec:   6936.21
Transfer/sec:      1.84MB

static-ex 30s 10 connections

Running 30s test @ http://localhost:8080/ping
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.96ms   26.66ms 202.38ms   87.04%
    Req/Sec     3.48k     1.48k    7.91k    67.83%
  205060 requests in 30.05s, 54.37MB read
Requests/sec:   6823.81
Transfer/sec:      1.81MB

jab-ex 30s 10 connections

Running 30s test @ http://localhost:8080/ping
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    13.91ms   26.52ms 212.59ms   86.86%
    Req/Sec     3.49k     1.53k    7.35k    67.48%
  205055 requests in 30.04s, 54.36MB read
Requests/sec:   6825.60
Transfer/sec:      1.81MB
