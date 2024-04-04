all: models.json

models.json: clean
	cargo run --release -- meta/models.yaml models.json

clean:
	rm models.json &> /dev/null || true
