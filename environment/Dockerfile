FROM rustlang/rust:nightly

ENV IMG_SHA256="cc9bf08794353ef57b400d32cd1065765253166b0a09fba360d927cfbd158088"

RUN apt update && \
    apt-get install uidmap libseccomp-dev && \
    mkdir /app && \
    curl -fSL "https://github.com/genuinetools/img/releases/download/v0.5.11/img-linux-amd64" -o "/app/img" && \
    echo "${IMG_SHA256}  /app/img" | sha256sum -c - && \
    chmod a+x "/app/img" && \
    groupadd -g 1000 img && \
    useradd -d /src -g 1000 -u 1000 -M img && \
    mkdir /work && \
    chown img:img /work
