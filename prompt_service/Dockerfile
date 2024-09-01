FROM docker.io/python:3.10-bullseye AS built_rust

RUN wget --output-document rustup-init https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init

RUN chmod +x rustup-init

RUN ./rustup-init -y --profile minimal

ENV PATH="/root/.cargo/bin:${PATH}"
ENV DOCKER_IMAGE=1

RUN rustup update && rustup install 1.80.1 && rustup default 1.80.1

WORKDIR /built_app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

FROM docker.io/python:3.10-bullseye AS python_requirements

RUN pip3 install --user pipenv==2024.0.1

ENV PATH="/root/.local/bin:${PATH}"

COPY Pipfile .
COPY Pipfile.lock .
RUN pipenv requirements > requirements.txt

FROM docker.io/python:3.10-bullseye

COPY --from=built_rust /built_app/target/release/prompt_service .
COPY --from=python_requirements requirements.txt .

RUN pip3 install -r requirements.txt
