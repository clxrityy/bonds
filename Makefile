# ----------------------------------------
# Variables
#	----------------------------------------
VERSION ?= v0.1.0
DOCS_PROFILE ?= strict
TAG ?= $(VERSION)

# ---------------------------------------
# Self-documenting help target.
# Parses ## comments on each target line.
# ---------------------------------------
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''

	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
    | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ---------------------------------------
# Setup targets.
# ---------------------------------------
setup-docs-venv: ## Setup .venv	for documentation generation
	@python3 -m venv .venv
	@.venv/bin/pip install -r docs/requirements.txt
	bash -c "source .venv/bin/activate"

# ---------------------------------------
# Testing targets.
# ---------------------------------------
test: ## Run all tests
	cargo test --workspace

test-core: ## Run tests for the core library
	cargo test -p bonds-core

test-cli: ## Run tests for the CLI
	cargo test -p bonds-cli

test-docs:	## Run documentation tests for the workspace
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
	cargo test --workspace --doc

test-docs-release: ## Trigger the documentation release workflow with test inputs. Usage: make test-docs-release VERSION=v0.1.0 DOCS_PROFILE=strict
	@act workflow_dispatch -W .github/workflows/docs-release.yml \
 --input version=$(VERSION) \
 --input include_api=true \
 --input include_guides=true \
 --input publish_latest=false \
 --input profile=$(DOCS_PROFILE) \
 --input dry_run=true

test-publish:	## Trigger the publish workflow with test inputs
	@act workflow_dispatch -W .github/workflows/publish.yml --input target=all --input dry_run=true

test-publish-core:	## Trigger the publish workflow for the core package with test inputs
	@act workflow_dispatch -W .github/workflows/publish.yml --input target=core --input dry_run=true

test-publish-cli:	## Trigger the publish workflow for the CLI package with test inputs
	@act workflow_dispatch -W .github/workflows/publish.yml --input target=cli --input dry_run=true

# ---------------------------------------
# Linting targets.
# ---------------------------------------
lint: ## Run all linters
	cargo fmt --all --check
	cargo clippy --workspace

lint-fix: ## Run all linters and fix issues
	cargo fmt --all
	cargo clippy --workspace --fix

lint-actions:	## Run linters with GitHub Actions annotations
	@actionlint -config-file .github/actionlint.yaml -verbose

# ---------------------------------------
# Build targets.
# ---------------------------------------
build: ## Build all packages
	cargo build --workspace

# build-release: ## Build all packages in release mode
#	cargo build --workspace --release

# ---------------------------------------
# Documentation targets.
# ---------------------------------------
docs-dev: setup-docs-venv ## Build documentation for all packages
	@rm -rf docs/site
	SITE_URL=$${SITE_URL:-http://127.0.0.1:4173/} .venv/bin/mkdocs build --strict -f docs/mkdocs.yml
	cargo doc --workspace --no-deps
	mkdir -p docs/site/api
	@cp -R target/doc/* docs/site/api/

#	---------------------------------------
# Release targets.
#	---------------------------------------
tag-release: ## Tag + push a release for the current version. Usages: `make tag-release TAG=v0.1.0` `make tag-release TAG=core-v0.1.0` `make tag-release TAG=cli-v0.1.0`
	@set -euo pipefail; \
		if ! git diff --quiet || ! git diff --cached --quiet; then \
				echo "Working tree has uncommitted changes; commit or stash first."; \
    exit 1; \
		fi; \
   if git rev-parse -q --verify "refs/tags/$(TAG)" >/dev/null; then \
    echo "Tag $(TAG) already exists locally."; \
    exit 1; \
  fi; \
  if git ls-remote --exit-code --tags origin "refs/tags/$(TAG)" >/dev/null 2>&1; then \
    echo "Tag $(TAG) already exists on origin."; \
    exit 1; \
  fi; \
  echo "Creating and pushing tag $(TAG)"; \
  git tag -a "$(TAG)" -m "Release $(TAG)"; \
  git push origin "$(TAG)"

# ---------------------------------------
# Utility targets.
# ---------------------------------------
show-tag: ## Show the tag that would be used
	@echo $(TAG)

# ---------------------------------------
# Clean targets.
# ---------------------------------------
clean: ## Clean build artifacts and documentation
	cargo clean
	rm -rf docs/site
	rm -rf .venv