# RSS List CLI
list your feeds using a `rust` based CLI.


https://github.com/Dpbm/rss-list/assets/75098594/44ed53a1-82e2-49db-bc9e-3b595b486a57

## Install

To install `rss-list` run the command below:

```bash
cargo install rss-list
```


## Arguments

| short | long | description |
|-------|------|-------------|
| -u    |--url | url from a feed |
| -f    |--file| file with a list of feeds urls|
| -l    |--limit| limit of recent posts for each url|

## Usage
```bash
rss-list -u <FEED_URL> -f <TXT_FILE> -l <POSTS_LIMIT>
or
rss-list --url <FEED_URL> --file <TXT_FILE> --limit <POSTS_LIMIT>
```

Example:

```bash
rss-list -u https://css-tricks.com/feed -l 20
rss-list -f ./feed.txt 
```

`Note: in your file, feed.txt, make sure that each line has only an url, and there are no empty lines!`


## Dev

For development use `cargo` for manage dependencies

To build the project just run:
```bash
cargo build
```

## Contributing 

If you want to help this project, just open and Issue and if your idea was approved, open a pull request

