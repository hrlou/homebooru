#! REPLACE WITH CARGO-MAKE

# commands
SQLITE=sqlite3
CARGO=cargo
TRUNK=trunk
SEA=sea-orm-cli
# files
ENTITY=src/db/entity
DATABASE=data/db/homebooru.db
UP=docs/schema/sql/up.sql
DOWN=docs/schema/sql/down.sql

.PHONY: r b
r: run
b: build

.PHONY: all run clean
all: build
build-client:
	$(TRUNK) build

build-server: 
	$(CARGO) build

build: build-client build-server
run: build
	$(CARGO) run

# codegen
entity: up
	$(SEA) generate entity -o $(ENTITY) -s $(UP) --with-serde both

# sql
down:
	@echo Database is going DOWN!
	@$(SQLITE) $(DATABASE) < $(DOWN)

up: down
	@echo Database is going UP!
	@$(SQLITE) $(DATABASE) < $(UP)

clean: down
	$(TRUNK) clean
	$(CARGO) clean