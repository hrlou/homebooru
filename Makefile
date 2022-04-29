# commands
SQLITE=sqlite3
CARGO=cargo
TRUNK=trunk
SEA=sea-orm-cli
# files
ENTITY=src/db/entity
DATABASE=data/db/homebooru.db
UP=data/up.sql
DOWN=data/down.sql

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

# sql
down: $(DATABASE)
	$(SQLITE) $(DATABASE) < $(DOWN)

up: down
	$(SQLITE) $(DATABASE) < $(UP)

entity: up
	$(SEA) generate entity -o $(ENTITY) -s $(UP) --with-serde both

clean: down
	$(TRUNK) clean
	$(CARGO) clean