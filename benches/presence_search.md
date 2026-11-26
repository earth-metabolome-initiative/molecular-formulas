# Element and isotope lookup benchmark

Return every matching compound ID in ascending order. Compare three paths: reading and parsing each source formula once in Rust, searching stored element/isotope pairs with only their document-first primary key, and running the same typed query with the `(element, mass, document_id)` lookup index. Each output is checked against independently annotated formulas outside the clock. The raw path includes delivery of all source rows to the client. The database paths deliver only matching IDs.

The deterministic fixture assigns 6,000 document IDs to 12 formula strings, including `Fe2O3`, `FeF3`, `[13C]H4` and `[13C]C2H6`. Repeated compositions and explicit labels are deliberate. It does not estimate natural isotope prevalence. One warmup and eight measured reads per query rotate query order within each method. Sample SD describes repeated reads on one dataset. Methods run in fixed order and the final phase drops the lookup index. Sizes are allocated relation or page bytes on each database, excluding the source table. PostgreSQL relation sizes include their dependent indexes and storage forks.

## Reproduce

Use PostgreSQL 18 with the `molecular_formulas` extension from this branch installed, SQLite with JSON and `dbstat`, and the locked Rust dependencies. Choose a fresh database and a new SQLite file. The benchmark creates its tables and drops its lookup index in the supplied databases.

```sh
set -eu
directory=$(mktemp -d)
database="presence_benchmark_$$"
createdb "$database"
psql -X -v ON_ERROR_STOP=1 -d "$database" -c 'CREATE EXTENSION molecular_formulas'
cargo bench --locked --bench presence_search --features diesel-sqlite,diesel-postgres,diesel/postgres -- "$directory/presence.sqlite" "dbname=$database" > "$directory/results.tsv"
```

The output includes engine versions, the corpus seed, each query plan, physical sizes, per-read observations, mean and sample SD in microseconds. The [recorded run](presence_search/results.tsv) was measured on an AMD Ryzen Threadripper 3970X with a local PostgreSQL socket.
