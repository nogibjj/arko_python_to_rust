rust_install:
	cargo install --path .

rust_format:
	cargo fmt --quiet

rust_lint:
	cargo clippy --quiet

rust_test:
	cargo test --quiet

rust_run:
	cargo run --release

rust_build:
	cargo build --release

rust_release:
	cargo build --release

rust_all: format lint test run


install: 
	python -m venv testenv && \
		. testenv/bin/activate && \
		pip install --upgrade pip && \
		pip install -r requirements.txt

test: 
	. testenv/bin/activate && \
		python -m pytest -vv --cov=main --cov=scripts test_*.py

format:	
	. testenv/bin/activate && \
		black scripts/*.py 

lint: 
	. testenv/bin/activate && \
		ruff check scripts/*.py

container-lint: 
	docker run --rm -i hadolint/hadolint < .devcontainer/Dockerfile

refactor: format lint

deploy:
	.d testenv/bin/activate && \
		docker build -f .devcontainer/Dockerfile -t arko_cli_tool:latest . && \
		docker rm -f arko_cli_tool || true && \
		docker run -d --name arko_cli_tool -p 80:80 arko_cli_tool:latest
		echo "Deployment completed."

all: install format lint test