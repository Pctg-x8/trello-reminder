.PHONY: all

all: deploy.yml

deploy.yml: deploy.dhall
	dhall-to-yaml --file deploy.dhall --output deploy.yml
