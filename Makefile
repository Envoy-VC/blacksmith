.PHONY: start stop

start:
	cd docker-compose/blockscout && docker-compose -f blockscout.yml up -d

stop:
	cd docker-compose/blockscout && docker-compose -f blockscout.yml down
	rm -rf ./data
