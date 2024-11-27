# Variables
PYTHON_VENV = ./.pyvenv/bin/activate
UVICORN_APP = backend/python/app.py

# Targets
develop:
	.  $(PYTHON_VENV)	&&	cd	backend/rust_core	&&	maturin	develop	&&	cargo	build	--workspace

build:
	cargo	build	--workspace

test:
	cargo	test	--workspace

clean:
	cargo	clean	&&	rm	-rf	backend/python/.venv

run-python:
	$(PYTHON_VENV)	&&	python	backend/python/main.py

run-fastapi:
	$(PYTHON_VENV)	&&	uvicorn	app:app	--reload	--app-dir	backend/python
