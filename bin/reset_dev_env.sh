INPUT=../expanding.jpg
OUTPUT=../recovered.jpg

echo "Generating Drop for $INPUT..."

cd rust

GEN_OUTPUT=$(
	cargo run -- \
		-p 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
		gen \
		--input $INPUT
)

echo $GEN_OUTPUT

ADDRESS=$(echo $GEN_OUTPUT | jq -r '.address')

echo "Dropped at $ADDRESS!"

PULL_OUTPUT=$(
	cargo run -- \
		-p 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
		pull \
		--address $ADDRESS \
		--output $OUTPUT
)

echo PUT_OUTPUT

echo "Recovered at $OUTPUT!"
