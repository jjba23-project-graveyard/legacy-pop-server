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

# pop-server-log-info: Logs informational messages in custom format with highlight.
define pop-server-log-info
	@printf "\n\033[1m[INFO] $(shell date --rfc-3339=seconds) - %s\033[0m\n" $(1)
endef

run:
	RUST_LOG=debug nix develop -c cargo run

fmt:
	cargo fix --allow-dirty --bin "pop-server"
	cargo clippy --fix --allow-dirty
	find . -maxdepth 8 -name "*.rs" -type f -exec rustfmt --edition 2021 {} \;
test: fmt
	cargo test

test-register-behavior:
	curl -v -X POST \
		--header "content-type: application/json" \
		--header "x-pop-server-request-path: /some-endpoint" \
		--header "x-pop-server-request-method: GET" \
		--header "x-pop-server-response-content-type: application/json" \
		--header "x-pop-server-status: 200" \
		--data '{"meta":"xyz","data":"xyz"}' \
		http://localhost:50088/api/v2/pop-server/register-behavior
	@echo ""
test-fetch-behavior:
	curl -v -X GET http://localhost:50088/some-endpoint
	@echo ""
test-default-behavior:
	curl -v -X GET http://localhost:50088/whatever
	@echo ""
test-scenario:
	make test-default-behavior
	make test-register-behavior
	make test-fetch-behavior


container-build:
	podman build -t jjba23/pop-server:$(CONTAINER_TAG) .
container-run:
	podman run jjba23/pop-server:$(CONTAINER_TAG)
container-push:
	podman push jjba23/pop-server:$(CONTAINER_TAG)


# publish-manual: Generates and publishes the manual from Org files using Emacs.
publish-manual:
	$(call pop-server-log-info,"generating readme for pop-server")
	emacs -Q --batch \
		-l $(shell realpath ./src/pop-server-manual.el) \
		--funcall pop-server/publish-readme
