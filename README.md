# jsonJar
api service that stores full or part of the rest json payloads as a file

send a json payload to:
- `/raw` to log a pretty printed json payload
- `/csv` to write a json payload to csv file based on the config

## config
configuration is formed from the following environment variables:
- `COLUMNS` - the number of columns to save in the csv file
- `COLUMN_<X>` - the keys chain for the X column. Eg: `COLUMN_1=foo.bar.baz` will save the value of `foo.bar.baz` in the first column
- `DELIMITER` - the delimiter to use in the csv file. By default `,`
- `CSV_FILE` - the optional path to the csv file to write to. By default the received line will be only logged
- `RUST_LOG` - the log level. By default `info`