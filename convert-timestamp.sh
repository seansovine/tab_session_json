#!/usr/bin/env bash

# Proper unix timestamp.
timestamp=1694101648
# Timestamp from JSON file.
timestamp=1694104295853

# Approach suggestion from Google AI:
formatted_date=$(date -d "@$timestamp" +"%Y-%m-%d %H:%M:%S")
echo $formatted_date

## NOTE: We used this to figure out that the timestamps in
## the file are in milliseconds. We are timestamps expecting
## in seconds in our Rust code, so we will convert.
