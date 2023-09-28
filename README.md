# jsonJar
api service that stores full or part of the rest json payloads as a file

send a json payload to:
- POST `/raw` to log a pretty printed json payload
- POST `/csv` to write a json payload to csv file based on the config
- GET `/csv` to get a csv file content
- GET `/health` to get a health status

## config
configuration is formed from the following environment variables:
- `COLUMNS` - the number of columns to save in the csv file
- `COLUMN_<X>` - the keys chain for the X column. Eg: `COLUMN_1=foo.bar.baz` will save the value of `foo.bar.baz` in the first column
- `DELIMITER` - the delimiter to use in the csv file. By default `,`
- `CSV_FILE` - the optional path to the csv file to write to. By default, the received line will be only logged
- `RUST_LOG` - the log level. By default `info`
- `PORT` - the port to listen on. By default `8080`

## quick test

    docker run -it --rm -p 8080:8080 -e COLUMNS=1 -e COLUMN_0=test ghcr.io/j0rsa/json-jar