# pop-server - a puppet server, at your command

# Copyright (C) 2025 Josep Bigorra (jjba23) <jjbigorra@gmail.com>

# pop-server is free software: you can redistribute it and/or modify
# it under the terms of the Lesser GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# pop-server is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# Lesser GNU General Public License for more details.

# You should have received a copy of the Lesser GNU General Public License
# along with pop-server.  If not, see <https://www.gnu.org/licenses/>.

#
# Build image
#
FROM docker.io/library/rust:1.84 as build

WORKDIR /app

COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src

RUN cargo build --target x86_64-unknown-linux-gnu --release

#
# Release image
#
FROM docker.io/library/rust:1.84

COPY --from=build /app/target/x86_64-unknown-linux-gnu/release/pop-server /app/pop-server

EXPOSE 50088

CMD ["/app/pop-server"]
