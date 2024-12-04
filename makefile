
TAILWIND_INPUT=styles/tailwind.css
TAILWIND_OUTPUT=static/main.css
AWS_URL=ubuntu@ec2-3-9-146-191.eu-west-2.compute.amazonaws.com
AWS_KEY=~/certs/aws-personal-website.pem

build_and_deploy_local: build
	docker-compose up

build_and_deploy: build deploy

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
	docker save -o website.tar website
	docker save -o db.tar db
	gzip website.tar
	gzip db.tar

tailwind:
	pnpm dlx tailwindcss -i $(TAILWIND_INPUT) -o $(TAILWIND_OUTPUT)

clean:
	cargo clean
	rm $(TAILWIND_OUTPUT)
