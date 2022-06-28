# Compile developing version of contracts.
dev:
	capsule build

# Compile releasing version of contracts.
release:
	capsule build --release

# Deploy contracts to the testnet of CKB.
deploy-testnet:
	API_URL=http://127.0.0.1:8224 capsule deploy --api $API_URL --env dev

# Deploy contracts to the mainnet of CKB.
deploy-mainnet:
	API_URL=http://127.0.0.1:8114 capsule deploy --api $API_URL --env production
