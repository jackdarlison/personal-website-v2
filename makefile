include $(PWD)/.env
TAILWIND_INPUT=styles/tailwind.css
TAILWIND_OUTPUT=static/main.css

build_and_deploy_local: build
	docker-compose down || true
	docker-compose up

build_and_deploy: build save deploy

deploy:
	ssh -i $(AWS_KEY) $(AWS_URL) "mkdir -p ~/website/"
	ssh -i $(AWS_KEY) $(AWS_URL) "cd ~/website/ && (docker-compose down || true)"
	scp -i $(AWS_KEY) website.tar.gz db.tar.gz .env compose.yaml $(AWS_URL):~/website/
	ssh -i $(AWS_KEY) $(AWS_URL) "cd ~/website/ && docker load < website.tar.gz"
	ssh -i $(AWS_KEY) $(AWS_URL) "cd ~/website/ && docker load < db.tar.gz"
	ssh -i $(AWS_KEY) $(AWS_URL) "cd ~/website/ && docker-compose up"

build: tailwind
	mkdir -p app
	docker build -f dockerfile.website -t website .
	docker build -f dockerfile.db -t db .

save:
	docker save -o website.tar website
	docker save -o db.tar db
	gzip -f website.tar
	gzip -f db.tar

tailwind:
	pnpm dlx tailwindcss -i $(TAILWIND_INPUT) -o $(TAILWIND_OUTPUT)

clean:
	cargo clean
	rm $(TAILWIND_OUTPUT)
