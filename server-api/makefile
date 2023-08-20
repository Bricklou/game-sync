.PHONY: migrate-run
migrate-run:
	sea-orm-cli migrate up -d ./projects/migration/

.PHONY: migrate-rollback
migrate-rollback:
	sea-orm-cli migrate down -d ./projects/migration/


.PHONY: migrate-reset
migrate-reset:
	sea-orm-cli migrate reset -d ./projects/migration/

.PHONY: migrate-refresh
migrate-refresh:
	sea-orm-cli migrate refresh -d ./projects/migration/

.PHONY: generate-entities
generate-entities:
	sea-orm-cli generate entity -o ./projects/app/src/entities \
		--with-serde both --serde-skip-hidden-column --serde-skip-deserializing-primary-key