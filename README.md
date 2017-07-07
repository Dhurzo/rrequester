# RRequester #

RRequester officially it's a sofware made for testing rests APIs and make load test with diferent arguments taken from a file.

Unofficially I had to test an Rest API for work and I wanted to use Rust :D

Status -> Lot of code crap and nearly abandonware


### HTTP Methods Support ###

* GET
* POST
* PUT

### USAGE ###

Help: ./rrequester -h :

Usage:
    ./target/debug/requester [OPTIONS]
```
Execute http requests to test REST services

optional arguments:
  -h,--help             show this help message and exit
  -u,--url URL          Url address
  -a,--auth AUTH        Basic auth
  -f,--argument_file ARGUMENT_FILE
                        Argument csv format file
  -w,--workers WORKERS  Number of workers
  -j,--jobs JOBS        Number of jobs
  -c,--headers HEADERS  Headers in format header:value,header:value,...
  -m,--method METHOD    Method (GET, POST or PUT) default GET
```

### TODO: ###

* Error handling
* ~~More wildcards replacement~~
* ~~Reduce methods complexitiy~~
* ~~POST / PUT Request~~
* ~~Modularity~~
* Testing
