FROM scratch

LABEL authors="red.avtovo@gmail.com"

COPY json-jar .

ENV RUST_LOG=info

CMD ["./json-jar"]