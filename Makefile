all: models.json

models.json: clean
	cat meta/models.yaml | yq > models.json

clean:
	rm models.json &> /dev/null || true
