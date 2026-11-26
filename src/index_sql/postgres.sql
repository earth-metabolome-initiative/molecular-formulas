CREATE TABLE mf_documents (
    id BIGINT PRIMARY KEY,
    formula TEXT NOT NULL
);

CREATE TABLE mf_postings (
    document_id BIGINT NOT NULL,
    element INTEGER NOT NULL,
    mass INTEGER NOT NULL,
    PRIMARY KEY (document_id, element, mass)
);
CREATE INDEX mf_postings_lookup ON mf_postings(element, mass, document_id);

CREATE FUNCTION mf_maintain_presence() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'TRUNCATE' THEN
        TRUNCATE mf_postings;
        RETURN NULL;
    END IF;

    IF TG_OP IN ('UPDATE', 'DELETE') THEN
        DELETE FROM mf_postings WHERE document_id = OLD.id;
    END IF;

    IF TG_OP IN ('INSERT', 'UPDATE') THEN
        INSERT INTO mf_postings (document_id, element, mass)
        SELECT NEW.id, (term ->> 0)::integer, (term ->> 1)::integer
        FROM jsonb_array_elements(mf_presence(NEW.formula)::jsonb) AS term;
    END IF;
    RETURN NULL;
END;
$$;

-- Resolve trigger dependencies in the installed corpus and extension schemas.
DO $$
DECLARE
    projection_schema name;
BEGIN
    SELECT pg_namespace.nspname INTO STRICT projection_schema
    FROM pg_extension JOIN pg_namespace ON pg_namespace.oid = pg_extension.extnamespace
    WHERE extname = 'molecular_formulas';
    EXECUTE format('ALTER FUNCTION mf_maintain_presence() SET search_path TO pg_catalog, %I, %I, pg_temp',
                   current_schema(), projection_schema);
END;
$$;

CREATE TRIGGER mf_documents_write AFTER INSERT OR UPDATE OR DELETE ON mf_documents
    FOR EACH ROW EXECUTE FUNCTION mf_maintain_presence();
CREATE TRIGGER mf_documents_truncate AFTER TRUNCATE ON mf_documents
    FOR EACH STATEMENT EXECUTE FUNCTION mf_maintain_presence();
