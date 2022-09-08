use chrono::prelude::*;
use clap::Parser;
use rfc2047_decoder;

#[derive(Parser)]
#[clap(name = "Bertha")]
#[clap(author = "Wener <aegisa7280@gmail.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "Search mails with a specific keyword using IMAP.", long_about = None)]
struct Args {
    //IMAP server's hostname
    #[clap(long, value_parser)]
    imap_hostname: String,
    //IMAP username
    #[clap(long, value_parser)]
    imap_username: String,
    //IMAP password
    #[clap(long, value_parser)]
    imap_password: String,
    //The keyword that you want to search
    #[clap(short, long, value_parser)]
    keyword: String,
    //How many mails will be listed
    #[clap(short, long, value_parser, default_value_t = 1)]
    number: u32,
}

fn main() {
    let args = Args::parse();

    //Connect to IMAP server and login
    let tls = native_tls::TlsConnector::builder()
        .build()
        .expect("Error while building TLS Connector");
    let imap_client = imap::connect((&args.imap_hostname[..], 993), &args.imap_hostname, &tls)
        .expect(&format!(
            "Error while connecting to IMAP server {}",
            args.imap_hostname
        ));
    let mut imap_session = imap_client
        .login(&args.imap_username, &args.imap_password)
        .expect("Error while logging into IMAP server");

    //Search with SUBJECT
    imap_session
        .select("INBOX")
        .expect("Error while selecting INBOX");
    let search = imap_session
        .search(format!("SUBJECT {}", args.keyword))
        .expect(&format!(
            "Error while searching with SUBJECT {}",
            args.keyword
        ));

    //Extract id and date from the search result
    let mut mails = Vec::with_capacity(search.len());
    for id in search {
        //Get headers and convert every line into a &str in the vector
        let header = imap_session
            .fetch(format!("{id}"), "BODY[HEADER]")
            .expect(&format!("Error while fetching mail whose id={id}"));
        let header = if let Some(h) = header.iter().next() {
            h
        } else {
            panic!("Error while reading header of the mail whose id={id}");
        };
        let header = header.header().expect(&format!(
            "Error while reading header of the mail whose id={id}"
        ));
        let header = std::str::from_utf8(header).expect(&format!(
            "The header of the mail whose id={id} isn't valid utf-8",
        ));

        //Find Date and Subject in the header
        let header_lines: Vec<&str> = header.split(&['\r', '\n'][..]).collect();
        let mut date = String::from("");
        let mut subject = String::from("");
        let mut found_date = false;
        let mut found_subject = false;
        for line in header_lines {
            let line = String::from(line.trim());
            if line.starts_with("Date: ") {
                date = String::from(&line["Date: ".len()..]);
                found_date = true;
            } else if line.starts_with("Subject: ") {
                subject = String::from(&line["Subject: ".len()..]);
                found_subject = true;
            }
            if found_date && found_subject {
                break;
            }
        }
        if date == "" {
            panic!("Didn't find a Date in the header of the mail whose id={id}");
        }
        //We don't need to panic if we didn't find subject

        //Now, date is in RFC2822 format
        let date = DateTime::parse_from_rfc2822(&date)
            .expect(&format!("Error parsing date of the mail whose id={id}"));
        //And subject is in RFC2047 format
        let subject = rfc2047_decoder::decode(subject.as_bytes())
            .expect("Error parsing subject of the mail whose id={id}");
        mails.push((id, date, subject));
    }

    //After this sort, the first mail is the oldest,the latest is the newest
    mails.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    for (i, mail) in mails.iter().rev().enumerate() {
        if i < args.number as usize {
            println!("{} @ {}", mail.2, mail.1.to_string());
        }
    }
    imap_session.logout().expect("Error while logging out");
}