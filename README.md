# Bertha

Search mails with a specific keyword using IMAP.

## Usage

### Build

`cargo build --release`

### Cli options

```
USAGE:
    bertha [OPTIONS] --imap-hostname <IMAP_HOSTNAME> --imap-username <IMAP_USERNAME> --imap-password <IMAP_PASSWORD> --keyword <KEYWORD>

OPTIONS:
    -h, --help                             Print help information
        --imap-hostname <IMAP_HOSTNAME>    
        --imap-password <IMAP_PASSWORD>    
        --imap-username <IMAP_USERNAME>    
    -k, --keyword <KEYWORD>                
    -n, --number <NUMBER>                  [default: 1]
    -V, --version                          Print version information
```

### Note

If you are using QQMail, please use an [authorized code](https://service.mail.qq.com/cgi-bin/help?subtype=1&&no=1001607&&id=28) instead of password.

Current version doesn't include any optimization, it will send a IMAP request for every mail in the search result. So please don't let INBOX contains too many mails, or it will spend a lot of time on network IO. Version 0.2.0 will use tokio, which can reduce time wasting on network IO.