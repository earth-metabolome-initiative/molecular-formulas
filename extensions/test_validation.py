"""Run the same write contract against an installed PostgreSQL or SQLite extension."""

import argparse
from contextlib import closing
import sqlite3
from unittest import TestCase


def check_writes(connection, parameter, database_error, table_kind):
    check = TestCase()
    connection.execute(
        f"CREATE {table_kind}TABLE samples (formula TEXT CHECK (must_be_molecular_formula(formula)))"
    )
    insert = f"INSERT INTO samples VALUES ({parameter})"
    formulas = ("CuSO4.5H2O", "SO₄²⁻", "[13C]H4", "[Co(NH3)5Cl]Cl2", "C65535")
    for formula in formulas:
        connection.execute(insert, (formula,))
    connection.execute(insert, (None,))
    check.assertCountEqual(
        connection.execute("SELECT formula FROM samples").fetchall(),
        [(formula,) for formula in (*formulas, None)],
    )
    check.assertEqual(
        connection.execute("SELECT must_be_molecular_formula(NULL)").fetchone(), (None,)
    )
    check.assertEqual(
        connection.execute("SELECT must_be_molecular_formula('H2O')").fetchone(), (True,)
    )
    for malformed in ("", "not a formula", "(OH", "C65536"):
        with check.assertRaises(database_error, msg=malformed):
            connection.execute(insert, (malformed,))
    with check.assertRaisesRegex(database_error, "(?i)overflow"):
        connection.execute("UPDATE samples SET formula = 'C65536'")
    check.assertCountEqual(
        connection.execute("SELECT formula FROM samples").fetchall(),
        [(formula,) for formula in (*formulas, None)],
    )
    connection.execute(
        f"CREATE {table_kind}TABLE required_samples "
        "(formula TEXT NOT NULL CHECK (must_be_molecular_formula(formula)))"
    )
    with check.assertRaises(database_error):
        connection.execute("INSERT INTO required_samples VALUES (NULL)")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    database = parser.add_mutually_exclusive_group(required=True)
    database.add_argument("--sqlite-extension", metavar="PATH")
    database.add_argument("--postgres", metavar="CONNECTION_STRING")
    arguments = parser.parse_args()
    if arguments.sqlite_extension:
        with closing(sqlite3.connect(":memory:", isolation_level=None)) as connection:
            connection.enable_load_extension(True)
            connection.load_extension(arguments.sqlite_extension)
            connection.enable_load_extension(False)
            connection.execute("PRAGMA trusted_schema = OFF")
            check_writes(connection, "?", sqlite3.DatabaseError, "")
            with TestCase().assertRaises(sqlite3.DatabaseError):
                connection.execute("INSERT INTO samples VALUES (?)", (b"H2O",))
    else:
        import psycopg

        with psycopg.connect(arguments.postgres, autocommit=True) as connection:
            connection.execute("CREATE EXTENSION IF NOT EXISTS molecular_formulas")
            check_writes(connection, "%s", psycopg.DatabaseError, "TEMP ")
