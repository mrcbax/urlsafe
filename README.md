# URLSafe v0.1.0
A tool to de-fang and re-arm malicious urls.

Takes a malicious url:

```
http://example.com/malware.exe
```

And converts it to its defanged version:

```
hxxp://example[.]com/malware[.]exe
```

Can also be used to re-arm a link with the decode option which works in reverse.

Supports the following protocols:

- [x] HTTP/S
- [x] FTP

(if you would like other protocols to be added create an issue.)

This tool is best used with a bash or powershell script for batch jobs.


### Help:

```
urlsafe 0.1.0
encodes and decodes de-fanged URLs.

USAGE:
    urlsafe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --decode <URL>    Returns a malicious URL to its dangerous state.
    -e, --encode <URL>    De-fangs a malicious URL.
```
