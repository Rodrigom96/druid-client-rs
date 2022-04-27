DOCKER_COMPOSE=docker/docker-compose.yaml
DOCKER_NETWORK=docker_default

start_containers:
	docker-compose -f $(DOCKER_COMPOSE) up -d --build

clean_containers:
	docker-compose -f $(DOCKER_COMPOSE) down

test_local: start_containers wait_druid
	cargo test

wait_druid:
	docker run --rm --network $(DOCKER_NETWORK) \
		busybox /bin/sh -c "until nc -z druid 8888; do sleep 3; echo 'Waiting for Druid to come up...'; done"
