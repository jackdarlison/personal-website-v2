
BIN_NAME=website
APP_DIR=/app
TAILWIND_INPUT=styles/tailwind.css
TAILWIND_OUTPUT=static/main.css

build: tailwind
	mkdir -p app
	docker build -t website .

tailwind:
	pnpm dlx tailwindcss -i $(TAILWIND_INPUT) -o $(TAILWIND_OUTPUT)

clean:
	cargo clean
	rm $(TAILWIND_OUTPUT)
