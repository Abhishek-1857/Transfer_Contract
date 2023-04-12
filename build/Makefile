.PHONY: all start build-store init
BLOCKCHAIN_PID_FILE := blockchain_process.pid
CONTRACT_PID_FILE := contrcat_process.pid
REPO_URL = https://github.com/provenance-io/provenance
BRANCH = main
CONTRACT_REPO_URL = https://github.com/Oasis-Pro-Inc/provenancesto
CONTRACT_BRANCH = development
export CODE_ID := my-code-id
export NAME := name
DENOM ?= nhash
MIN_FLOOR_PRICE ?= 1905
CHAIN_ID ?= testing
BUILDDIR ?= ${PWD}/build

start:
	@if test -f $(BLOCKCHAIN_PID_FILE); then \
		echo "Process already running with PID $$(cat $(BLOCKCHAIN_PID_FILE))" ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced start ; \
	else \
		echo "Starting process..." ; \
		./myprocess & echo $$! > $(BLOCKCHAIN_PID_FILE) ; \
		mkdir provnode ; \
		cd provnode ; \
		git clone --depth=1 --branch=$(BRANCH) $(REPO_URL) ; \
		cd provenance ; \
		make install ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced init --chain-id=$(CHAIN_ID) testing --custom-denom=$(DENOM); \
		provenanced -t --home $(BUILDDIR)/run/provenanced keys add validator --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced keys add signer1 --keyring-backend test  ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced keys add signer2 --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced keys add signer3 --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-root-name validator pio --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-root-name validator pb --restrict=false --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-root-name validator io --restrict --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-root-name validator provenance --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-account validator 10000000000000000000000000000000000000000000000$(DENOM)  --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-account signer1 1000000000000000000000000000000000000000000hotdogcoin  --keyring-backend test ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced gentx validator 1000000000000000$(DENOM)  --keyring-backend test --chain-id=$(CHAIN_ID); \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-marker 100000000000000000000$(DENOM)  --manager validator --access mint,burn,admin,withdraw,deposit --activate --keyring-backend test; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-marker 100000000000000000000hotdogcoin  --manager signer1 --access mint,burn,admin,withdraw,deposit --activate --keyring-backend test; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-msg-fee /provenance.name.v1.MsgBindNameRequest 10000000000$(DENOM) ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-msg-fee /provenance.marker.v1.MsgAddMarkerRequest 100000000000$(DENOM) ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-msg-fee /provenance.attribute.v1.MsgAddAttributeRequest 10000000000$(DENOM) ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-msg-fee /provenance.metadata.v1.MsgWriteScopeRequest 10000000000$(DENOM) ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced add-genesis-custom-floor $(MIN_FLOOR_PRICE)$(DENOM) ; \
		cat $(BUILDDIR)/run/provenanced/config/genesis.json | jq '.app_state.auth.accounts[3].allow_forced_transfer = true' > tmp.json ; \
		mv tmp.json $(BUILDDIR)/run/provenanced/config/genesis.json ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced collect-gentxs ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced config set minimum-gas-prices ${MIN_FLOOR_PRICE}${DENOM} ; \
		provenanced -t --home $(BUILDDIR)/run/provenanced start ; \
	fi


store:
	provenanced -t tx wasm store ${PWD}/../target/wasm32-unknown-unknown/release/transfer.wasm --from validator --home $(BUILDDIR)/run/provenanced --chain-id=$(CHAIN_ID) --broadcast-mode block -y --gas 4000000 --gas-prices 1905nhash -o json > ${PWD}/wasm_storeInfo.json ; \



init:
	provenanced -t tx wasm instantiate $(CODE_ID) '{"name":"$(NAME)","signers":["tp1v0z84vq38kn3x995flws4h69mswg943jsznqaf","tp1g6z8fgzvu9g0d53q96h8djc78s5xzc22zg9pcn","tp1zkttwpw97wfskmfj7xy5fawzy3n4d7rg0s5dhn"]}' --label "custom-marker" --gas 4000000 --gas-prices 4500nhash --from validator --chain-id=$(CHAIN_ID) -t  --broadcast-mode block -y --home $(BUILDDIR)/run/provenanced --no-admin -o json> ${PWD}/contract_info.json ; \


add-request:
	provenanced -t --home $(BUILDDIR)/run/provenanced tx wasm execute tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8 '{"create_mint_request":{}}' --from validator --home $(BUILDDIR)/run/provenanced --chain-id=$(CHAIN_ID) --broadcast-mode block -y --gas 4000000 --gas-prices 1905nhash -o json > ${PWD}/create_result.json ; \
	echo "RESULT :::::::>>>>>>>> $$(jq -r '.raw_log' create_result.json)" ; \

approve-request:
	provenanced -t --home $(BUILDDIR)/run/provenanced tx wasm execute tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8 '{"approve_mint_request":{"proposal_id":"4"}}' --from signer3 --home $(BUILDDIR)/run/provenanced --chain-id=$(CHAIN_ID) --broadcast-mode block -y --gas 4000000 --gas-prices 1905nhash -o json > ${PWD}/approve_result.json ; \
	echo "RESULT :::::::>>>>>>>> $$(jq -r '.raw_log' approve_result.json)" ; \

mint:
	provenanced -t --home $(BUILDDIR)/run/provenanced tx wasm execute tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8 '{"mint":{"proposal_id":"4"}}' --from validator --home $(BUILDDIR)/run/provenanced --chain-id=$(CHAIN_ID) --broadcast-mode block -y --gas 4000000 --gas-prices 1905nhash -o json > ${PWD}/mint_result.json ; \
	echo "RESULT :::::::>>>>>>>> $$(jq -r '.raw_log' mint_result.json)"  \

fund :
	provenanced -t --home $(BUILDDIR)/run/provenanced tx bank send validator tp1v0z84vq38kn3x995flws4h69mswg943jsznqaf 10000000000nhash --from validator --chain-id=$(CHAIN_ID) --broadcast-mode block -y --gas 4000000 --gas-prices 1905nhash -o json | jq
	provenanced -t --home $(BUILDDIR)/run/provenanced tx bank send validator tp1g6z8fgzvu9g0d53q96h8djc78s5xzc22zg9pcn 10000000000nhash --from validator --chain-id=$(CHAIN_ID) --broadcast-mode block -y --gas 4000000 --gas-prices 1905nhash -o json | jq
	provenanced -t --home $(BUILDDIR)/run/provenanced tx bank send validator tp1zkttwpw97wfskmfj7xy5fawzy3n4d7rg0s5dhn 10000000000nhash --from validator --chain-id=$(CHAIN_ID) --broadcast-mode block -y --gas 4000000 --gas-prices 1905nhash -o json | jq