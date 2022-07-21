MAINNET_RPC_URL := http://100.112.21.76:8114
TESTNET_RPC_URL := http://100.112.21.76:8224

# Compile developing version of contracts.
dev:
	capsule build

# Compile releasing version of contracts.
release:
	capsule build --release

# Deploy contracts to the testnet of CKB.
deploy-testnet: release
	API_URL=$(TESTNET_RPC_URL) capsule deploy --api $(TESTNET_RPC_URL) --env dev --fee 1 --address ckt1qyqtc5p2xjjrpclpvlyz5fxmd7fr0v27hu6slun8lz

# Deploy contracts to the mainnet of CKB.
deploy-mainnet: release
	API_URL=$(MAINNET_RPC_URL) capsule deploy --api $(MAINNET_RPC_URL) --env production --fee 1 --address ckb1qyqpfhfzzdkwwjhw9gq8cu09gsq58k4hkvnqtfsp75
