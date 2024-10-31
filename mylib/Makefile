install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv test_python_files/test_*.py

check:
	python test_time.py
	git config --local user.email "action@github.com"; \
	git config --local user.name "Github Action"; \
	git add .; \
	git commit -m "Makefile git push"; \
	git push; \

format:	
	black *.py 

lint:
	ruff check *.py
		