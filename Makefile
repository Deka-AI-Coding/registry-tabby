all: models.json

models.json: clean
	cargo run -- meta/models.yaml models.json

clean:
	rm models.json &> /dev/null || true
