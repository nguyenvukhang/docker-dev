FROM ubuntu:22.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  zsh \
  git \
  cmake \
  ninja-build \
  pkg-config \
  build-essential \
  libaio-dev \
  libboost-all-dev \
  libbz2-dev \
  libdouble-conversion-dev \
  libdwarf-dev \
  libiberty-dev \
  liblz4-dev \
  liblzma-dev \
  libsnappy-dev \
  libsodium-dev \
  libssl-dev \
  libunwind-dev \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/\* /tmp/\* /var/tmp/\*

# pkg-config \
# binutils-dev \

# bison flex \
# libevent-dev \
# libgoogle-glog-dev \
# libgflags-dev \
#
# apt install zlib1g-dev libjemalloc-dev libelf-dev
