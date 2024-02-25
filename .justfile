run:
  just dev & just watch

dev:
  trunk serve --open

build: tailwindcss
  trunk build --release

tailwindcss:
  npx tailwindcss -i inputs.css -o public/tailwind.css

watch:
  npx tailwindcss -i inputs.css -o public/tailwind.css --watch
