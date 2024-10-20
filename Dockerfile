FROM docker.io/rust:slim
    MAINTAINER Linus Kirkwood <linuskirkwood@gmail.com>

COPY ./ /opt
WORKDIR /opt
RUN cargo build --release

ENTRYPOINT ["/bin/bash"]
