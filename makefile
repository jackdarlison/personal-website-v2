include $(PWD)/.env

build_and_deploy_local: build
	docker-compose down || true
	docker-compose up

build_and_deploy: build save deploy

deploy:
	ssh $(SERVER_URL) "mkdir -p ~/website/"
	ssh $(SERVER_URL) "cd ~/website/ && (docker compose down || true)"
	scp website.tar.gz db.tar.gz .env compose.yaml $(SERVER_URL):~/website/
	ssh $(SERVER_URL) "cd ~/website/ && docker load < website.tar.gz"
	ssh $(SERVER_URL) "cd ~/website/ && docker load < db.tar.gz"
	ssh $(SERVER_URL) "cd ~/website/ && rm *.tar.gz"
	ssh $(SERVER_URL) "cd ~/website/ && docker compose up"

build:
	mkdir -p app
	docker build -f dockerfile.website -t website .
	docker build -f dockerfile.db -t db .

save:
	docker save -o website.tar website
	docker save -o db.tar db
	gzip -f website.tar
	gzip -f db.tar

clean:
	cargo clean
