# ----------------------------------------
# Variables
#	----------------------------------------
VERSION ?= v0.1.8
VERSION_TARGET	?= all
SYNC_MAKEFILE ?= false
DOCS_PROFILE ?= strict
TAG ?= $(VERSION)
TARGET ?= $(VERSION_TARGET)

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

#	---------------------------------------
# Start/refresh/setup the development environment.
#	---------------------------------------

setup: install-deps	setup-docs	## Setup the development environment for the entire workspace

refresh: clean setup-docs setup-app build test app lint	## Clean, setup, build, test, and lint the entire workspace

# ----------------------------------------
# Installation	targets.
# ----------------------------------------
install-deps:	## Install dependencies for all packages
	cargo fetch --workspace
	@cd crates/app && pnpm install

# ---------------------------------------
# Setup targets.
# ---------------------------------------
setup-docs: ## Setup .venv	for documentation generation
	@python3 -m venv .venv
	@.venv/bin/pip install -r docs/requirements.txt
	bash -c "source .venv/bin/activate"

setup-app: ## Setup the Bonds desktop app
	@cd crates/app && pnpm install

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
	@cd	crates/app && pnpm lint

lint-fix: ## Run all linters and fix issues
	cargo fmt --all
	cargo clippy --workspace --fix
	@cd	crates/app && pnpm lint:fix

lint-actions:	## Run linters with GitHub Actions annotations
	@actionlint -config-file .github/actionlint.yaml -verbose

# ---------------------------------------
# Build targets.
# ---------------------------------------
build: ## Build all packages
	cargo build --workspace

# build-release: ## Build all packages in release mode
#	cargo build --workspace --release

# ----------------------------------------
# Pre targets
#	----------------------------------------
pre-commit: lint test build ## Run all pre-commit checks (linting, tests, and build) to ensure code quality before committing

pre-tag: pre-commit show-tag show-target ## Run pre-commit checks and show the tag that would be used for the release

pre-publish: pre-tag test-docs-release test-publish ## Run pre-commit checks, test the documentation release workflow, and test the publish workflow to ensure everything is in order before publishing
	@echo "Pre-publish checks passed!"
	@echo "Run: make publish VERSION=$(VERSION) TARGET=$(TARGET) to publish the release."

# ---------------------------------------
# Documentation targets.
#
# TODO: The docs script will integrate with the app from `crates/app` to generate guides and API docs. For now, it just builds the Rust docs and copies them to the docs/site/api folder.
# ---------------------------------------
docs: setup-docs ## Build documentation for all packages
	@rm -rf docs/site
	SITE_URL=$${SITE_URL:-http://127.0.0.1:4173/} .venv/bin/mkdocs build --strict -f docs/mkdocs.yml
	cargo doc --workspace --no-deps
	mkdir -p docs/site/api
	@cp -R target/doc/* docs/site/api/

# ---------------------------------------
# Utility targets.
# ---------------------------------------
show-tag: ## Show the tag that would be used
	@echo $(TAG)

show-target: ## Show the version that would be used
	@echo $(TARGET)

# ---------------------------------------
# Clean targets.
# ---------------------------------------
clean: ## Clean build artifacts and temporary development files
	cargo clean
	rm -rf docs/site
	rm -rf .venv
	rm -rf crates/app/web/dist
	rm -rf crates/app/node_modules

#	---------------------------------------
# Release targets.
#	---------------------------------------
version-release: version	## Update version, commit, push, trigger publish workflow, create release commits, and tag the release. Usage: `make version-release VERSION=v0.1.0 VERSION_TARGET=core SYNC_MAKEFILE=true`
	@git add .
	@git commit -m "Bump to $(VERSION)"
	@git push
	@make tag-release TAG=$(VERSION)
	@make publish VERSION=$(VERSION) TARGET=$(VERSION_TARGET || all)

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
docs-release: ## Trigger the documentation release workflow. Usage: `make docs-release VERSION=v0.1.0 DOCS_PROFILE=strict`
	@act workflow_dispatch -W .github/workflows/docs-release.yml \
	--input version=$(VERSION) \
	--input include_api=true \
	--input include_guides=true \
	--input publish_latest=false \
	--input profile=$(DOCS_PROFILE)

publish: ## Trigger the publish workflow. Usage: `make publish VERSION=v0.1.0` `make publish TARGET=core VERSION=v0.1.0` `make publish TARGET=cli VERSION=v0.1.0`
	@act workflow_dispatch -j publish --input target=$(TARGET) --input version=$(VERSION)

version: ## Update crate version(s) locally. Usage: `make version VERSION=v0.0.0 VERSION_TARGET=all SYNC_MAKEFILE=true`
	@python3 scripts/versioner.py --version $(VERSION) --target $(VERSION_TARGET) --update-makefile $(SYNC_MAKEFILE == true && "--update-makefile" || "")

release-commits: ## Create release commits for the given version. Usage: `make release-commits VERSION=v0.1.0`
	@bash scripts/release-commits.sh $(VERSION)

# ---------------------------------------
# App targets.
# ---------------------------------------

app: ## Build the Bonds desktop app
	@cd crates/app && pnpm build

app-dev: ## Run the Bonds desktop app in development mode
	@cd crates/app && pnpm tauri dev

app-preview: ## Run the Bonds desktop app preview server
	@cd crates/app && pnpm build && pnpm preview