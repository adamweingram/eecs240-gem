FROM gcr.io/gem5-test/ubuntu-20.04_all-dependencies:v22-1

### Install Base Dependencies ###
RUN apt-get update
RUN apt-get install -y curl wget git

### Set up and install Rust ###
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust-install.sh

RUN chmod +x /tmp/rust-install.sh

RUN sh /tmp/rust-install.sh --default-toolchain stable -y

RUN stat "$HOME/.cargo/bin"

ENV PATH="$HOME/.cargo/bin:$PATH"

ENV RUST_BACKTRACE=1

### Set up Gem5 and Bench ###
WORKDIR /root

# Clone gem5 repo
RUN git clone https://github.com/gem5/gem5.git gem5

# Move to gem5 repo after cloning
WORKDIR /root/gem5
RUN git checkout v22.1.0.0

# Build Gem
RUN scons build/X86/gem5.opt -j 14

# Copy over and extract the bench data
WORKDIR /root
COPY ./bench.tar.bz2 /root
RUN tar -xvjf bench.tar.bz2

# Install other utilities
RUN apt-get install -y sqlite

# Will allow live interaction after launch
ENTRYPOINT ["/bin/bash"]
