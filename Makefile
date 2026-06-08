.PHONY: build-rust build-desktop build-mobile build-docker dev-up test-rust clean

build-rust:
	cargo build --release --workspace

build-desktop:
	cd apps/desktop && pnpm install --frozen-lockfile && pnpm tauri build

build-mobile:
	cd apps/mobile && flutter pub get && flutter build apk --release

build-appbundle:
	cd apps/mobile && flutter pub get && flutter build appbundle --release

build-web:
	cd apps/web && npm install && npm run build

build-docker:
	docker build -f deploy/Dockerfile.server    -t borderless-server    .
	docker build -f deploy/Dockerfile.signaling -t borderless-signaling .
	cd apps/web && docker build -t borderless-web .

dev-up:
	cd deploy && docker compose up -d db redis coturn

dev-down:
	cd deploy && docker compose down

test-rust:
	cargo test --all

clean:
	cargo clean
	rm -rf apps/mobile/build apps/desktop/src-tauri/target apps/web/.next
