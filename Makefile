DIRS := \
    . \
    api-server-rest \
    attestation-agent/coco_keyprovider \
    attestation-agent/kbc \
    attestation-agent/app \
    attestation-agent/attester \
    attestation-agent/lib \
    attestation-agent/deps/crypto \
    attestation-agent/deps/sev \
    attestation-agent/deps/resource_uri \
    attestation-agent/test-binaries \
    attestation-agent/kbs_protocol \
    image-rs \
    image-rs/libs/test-utils \
    confidential-data-hub/kms \
    confidential-data-hub/secret \
    confidential-data-hub/hub

.PHONY: clean

clean:
	rm -Rf guest-components/

.PHONY: clone-repo
clone-repo:
	@if [ ! -d "guest-components" ]; then \
		git clone https://github.com/confidential-containers/guest-components.git; \
		cd guest-components; git checkout tags/v0.8.0; \
		cd ..; \
	fi
	@echo "Repo exists"

.PHONY: vendor
vendor: clone-repo
	@echo "Vendoring dependencies in guest-components..."; \
	cmd="cargo vendor"; \
	for dir in $(DIRS); do \
		cmd+=" -s $$dir/Cargo.toml"; \
   	done; \
	cd guest-components; \
	$$cmd

.PHONY: update_lock
update_lock: clone-repo
	@cd guest-components; \
	cargo generate-lockfile; \
	mv Cargo.lock ../app/.cargo/Cargo.lock

.PHONY: sync
sync:
	@rsync -av --delete */vendor app
