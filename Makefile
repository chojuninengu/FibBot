# Makefile for running the FibBot Dockerfile

# Target to build the project
build:
	docker build -t fibbot .

exec:
	if [ -z "$GITHUB_TOKEN" ] || [ -z "$GITHUB_REPOSITORY" ] || [ -z "$PR_NUMBER" ]; then \
		echo "Missing one of the required variables: GITHUB_TOKEN, GITHUB_REPOSITORY, PR_NUMBER"; \
	else \
		docker run -it --rm fibbot $1 $2; \
	fi

# Combined target to build and execute the bot
start:
	$(MAKE) build
	$(MAKE) exec

.PHONY: build exec start