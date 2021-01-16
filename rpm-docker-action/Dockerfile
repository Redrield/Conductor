FROM fedora:latest

# Install devel tools
RUN dnf groupinstall 'Development Tools' -y
RUN dnf install libudev-devel webkit2gtk3-devel fedora-packager rpmdevtools -y
# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source $HOME/.cargo/env
# Install packager tool
RUN cargo install cargo-rpm

# Set entrypoint
COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]


