run: tailwindcss
  trunk serve --open

build: tailwindcss
  trunk build --release

tailwindcss:
  npx tailwindcss -i inputs.css -o public/styles.css

watch:
  npx tailwindcss -i inputs.css -o public/styles.css --watch
