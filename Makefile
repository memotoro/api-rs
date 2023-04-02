TAG="latest"

SERVICE="api-rs"

REPO="memotoro"

.PHONY: default
default: local-run

.PHONY: local-run
local-run:
	cargo clippy
	cargo run

.PHONY: up
up:
	docker compose up

.PHONY: down
down:
	docker compose down --volumes

.PHONY: docker-image
docker-image:
	docker build -t $(REPO)/$(SERVICE):$(TAG) -f Dockerfile .

.PHONY: docker-push
docker-push:
	docker push $(REPO)/$(SERVICE):$(TAG)
