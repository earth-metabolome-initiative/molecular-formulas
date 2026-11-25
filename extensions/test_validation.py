"""Exercise molecular formula constraints on an installed PostgreSQL extension."""

import argparse
import psycopg
from unittest import TestCase


def check_writes(connection):
    check = TestCase()
    connection.execute(
        "CREATE TEMP TABLE samples (formula TEXT CHECK (must_be_molecular_formula(formula)))"
    )
    insert = "INSERT INTO samples VALUES (%s)"
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
        with check.assertRaises(psycopg.DatabaseError, msg=malformed):
            connection.execute(insert, (malformed,))
    with check.assertRaisesRegex(psycopg.DatabaseError, "(?i)overflow"):
        connection.execute("UPDATE samples SET formula = 'C65536'")
    check.assertCountEqual(
        connection.execute("SELECT formula FROM samples").fetchall(),
        [(formula,) for formula in (*formulas, None)],
    )
    connection.execute(
        "CREATE TEMP TABLE required_samples "
        "(formula TEXT NOT NULL CHECK (must_be_molecular_formula(formula)))"
    )
    with check.assertRaises(psycopg.errors.NotNullViolation):
        connection.execute("INSERT INTO required_samples VALUES (NULL)")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--postgres", required=True, metavar="CONNECTION_STRING")
    arguments = parser.parse_args()
    with psycopg.connect(arguments.postgres, autocommit=True) as connection:
        connection.execute("CREATE EXTENSION IF NOT EXISTS molecular_formulas")
        check_writes(connection)
