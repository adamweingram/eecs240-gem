FROM gcr.io/gem5-test/ubuntu-20.04_all-dependencies:v22-1

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

# Will allow live interaction after launch
ENTRYPOINT ["/bin/bash"]
