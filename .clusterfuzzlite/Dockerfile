FROM gcr.io/oss-fuzz-base/base-builder-rust
# the base image pins a nightly older than the dependency graph builds on
ENV RUSTUP_TOOLCHAIN=nightly
RUN rustup toolchain install $RUSTUP_TOOLCHAIN --profile minimal
COPY . $SRC/molecular-formulas
WORKDIR $SRC/molecular-formulas
COPY ./.clusterfuzzlite/build.sh $SRC/
