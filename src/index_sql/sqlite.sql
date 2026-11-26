CREATE TABLE mf_documents (
    id BIGINT PRIMARY KEY CHECK (typeof(id) = 'integer'),
    formula TEXT NOT NULL
) WITHOUT ROWID;

CREATE TABLE mf_postings (
    document_id BIGINT NOT NULL,
    element INTEGER NOT NULL,
    mass INTEGER NOT NULL,
    PRIMARY KEY (document_id, element, mass)
) WITHOUT ROWID;
CREATE INDEX mf_postings_lookup ON mf_postings(element, mass, document_id);

CREATE TRIGGER mf_documents_insert AFTER INSERT ON mf_documents BEGIN
    -- REPLACE may displace an existing document without firing a delete trigger.
    DELETE FROM mf_postings WHERE document_id = NEW.id;
    INSERT INTO mf_postings (document_id, element, mass)
    SELECT NEW.id, json_extract(value, '$[0]'), json_extract(value, '$[1]')
    FROM json_each(mf_presence(NEW.formula));
END;

CREATE TRIGGER mf_documents_update AFTER UPDATE ON mf_documents BEGIN
    DELETE FROM mf_postings WHERE document_id IN (OLD.id, NEW.id);
    INSERT INTO mf_postings (document_id, element, mass)
    SELECT NEW.id, json_extract(value, '$[0]'), json_extract(value, '$[1]')
    FROM json_each(mf_presence(NEW.formula));
END;

CREATE TRIGGER mf_documents_delete AFTER DELETE ON mf_documents BEGIN
    DELETE FROM mf_postings WHERE document_id = OLD.id;
END;
