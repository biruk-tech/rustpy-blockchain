# Variables
PYTHON_VENV = ./.pyvenv/bin/activate
UVICORN_APP = backend/python/app.py

# Targets
build:
	cd	backend/rust_crypto	&&	cargo	build	--workspace

update:
	cd	backend/rust_crypto	&&	cargo	update	--workspace

run:
	cd	backend/rust_crypto	&&	cargo	run

test:
	cargo	test	--workspace

clean:
	cd	backend/rust_crypto	&& cargo	clean

run-python:
	$(PYTHON_VENV)	&&	python	backend/python/main.py

run-fastapi:
	$(PYTHON_VENV)	&&	uvicorn	app:app	--reload	--app-dir	backend/python
