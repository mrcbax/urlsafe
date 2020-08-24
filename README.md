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

Supports the following protocol URI schemes:

- [x] HTTP/S
- [x] FTP/S

Other schemes can be found [here](https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml). Any of these can be added, but will slow down processing time so only pertinent ones should be added. The other issue is that IANA has assigned hxxp and hxxps as actual "malicious" schemes, but has not created alternatives for other schemes (including FTP). I need a good way of converting these schemes to an inert form without breaking the specification or clobbering potential schemes. My current solution will be to wrap the scheme in square brackets; for example:

```
[nfs]://example[.]com/malware[.]exe
```

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
