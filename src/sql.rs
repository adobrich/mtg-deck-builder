pub const CREATE_DATABASE_TABLES_STMT: &str = r#"
    BEGIN;
    CREATE TABLE IF NOT EXISTS card_has_legality (
        card_id INTEGER NOT NULL,
        legality_id INTEGER NOT NULL,

        FOREIGN KEY (card_id) REFERENCES card(id) ON DELETE CASCADE,
        FOREIGN KEY (legality_id) REFERENCES legality(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS legality (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        card_id INTEGER NOT NULL,
        format_id INTEGER NOT NULL,
        status TEXT NOT NULL,

        CONSTRAINT no_duplicates UNIQUE (card_id, format_id),

        FOREIGN KEY (card_id) REFERENCES card(id) ON DELETE CASCADE,
        FOREIGN KEY (format_id) REFERENCES format(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS format (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE
    );

    CREATE TABLE IF NOT EXISTS deck_has_card (
        deck_id INTEGER NOT NULL,
        card_id INTEGER NOT NULL,

        FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE,
        FOREIGN KEY (card_id) REFERENCES card(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS decks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        format TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS card (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ability TEXT,
        artist TEXT,
        back_face_id INTEGER NOT NULL,
        cmc REAL NOT NULL,
        color_identity INTEGER NOT NULL,
        colors TEXT,
        flavor_text TEXT,
        image BLOB,
        is_foil INTEGER NOT NULL,
        language TEXT NOT NULL,
        local_count INTEGER NOT NULL DEFAULT 0,
        loyalty TEXT,
        mana_cost TEXT,
        multiface INTEGER NOT NULL,
        name TEXT NOT NULL,
        number TEXT NOT NULL,
        power TEXT,
        price REAL NOT NULL DEFAULT 0.0,
        promo INTEGER NOT NULL,
        rarity TEXT NOT NULL,
        scryfall_id TEXT NOT NULL,
        set_name TEXT NOT NULL,
        set_code TEXT NOT NULL,
        toughness TEXT,
        type_line TEXT NOT NULL,
        watermark TEXT,

        CONSTRAINT foils_are_distinct UNIQUE (scryfall_id, is_foil)
    );

    CREATE TABLE IF NOT EXISTS card_back_face (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ability TEXT,
        artist TEXT,
        card_id INTEGER NOT NULL UNIQUE,
        cmc REAL,
        color_identity INTEGER,
        colors TEXT,
        flavor_text TEXT,
        image BLOB NOT NULL,
        loyalty TEXT,
        mana_cost TEXT,
        name TEXT NOT NULL,
        power TEXT,
        toughness TEXT,
        type_line TEXT,
        watermark TEXT
    );

    CREATE TABLE IF NOT EXISTS sets (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        block_code TEXT,
        block_name TEXT,
        card_count INTEGER NOT NULL,
        code TEXT NOT NULL,
        name TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS rulings (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        card_id INTEGER NOT NULL,
        comment TEXT NOT NULL,
        oracle_id TEXT NOT NULL,
        published_at TEXT NOT NULL,
        source TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS config (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        first_run INTEGER NOT NULL,
        version TEXT NOT NULL
    );
    COMMIT;
"#;

pub const INSERT_CARD_STMT: &str = r#"
    INSERT INTO card (
        id,
        ability,
        artist,
        back_face_id,
        cmc,
        color_identity,
        colors,
        flavor_text,
        image,
        is_foil,
        language,
        local_count,
        loyalty,
        mana_cost,
        multiface,
        name,
        number,
        power,
        price,
        promo,
        rarity,
        scryfall_id,
        set_code,
        set_name,
        toughness,
        type_line,
        watermark
    )
    VALUES (
      ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15,
      ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27
    )
"#;

pub const INSERT_CARD_LEGALITY_STMT: &str = r#"
    INSERT INTO legality (card_id, format_id, status)
    VALUES (?1, (SELECT id FROM format WHERE name = ?2), ?3)"
"#;

pub const INSERT_CARD_HAS_LEGALITY_STMT: &str = r#"
    INSERT INTO card_has_legality (card_id, legality_id)
    VALUES (?1, ?2)
"#;

pub const INSERT_DECK_STMT: &str = r#"
    INSERT INTO decks (name, format) VALUES (?1, ?2)
"#;

pub const INSERT_CARD_TO_DECK_STMT: &str = r#"
    INSERT INTO deck_has_card (deck_id, card_id) VALUES (?1, ?2)
"#;

pub const INSERT_FORMAT_STMT: &str = r#"
    INSERT INTO format (name)
    VALUES (?)
"#;

pub const FETCH_ALL_LOCAL_CARDS_STMT: &str = r#"
    SELECT * FROM card
"#;

pub const FETCH_CARD_BY_ID_STMT: &str = r#"
    SELECT * FROM card WHERE id = ?
"#;

pub const FETCH_CARD_BACK_FACE_BY_ID_STMT: &str = r#"
    SELECT * FROM card_back_face WHERE id = ?
"#;

pub const FETCH_CARD_LEGALITIES_STMT: &str = r#"
    SELECT format.name, legality.status
    FROM card
    INNER JOIN card_has_legality
    ON card.id = card_has_legality.card_id
    INNER JOIN legality
    ON legality.id = card_has_legality.legality_id
    INNER JOIN format ON legality.format_id = format.id
    WHERE card.id = ?
    ORDER BY format.name
"#;

pub const FETCH_CARDS_BY_FORMAT_LEGALITY_STMT: &str = r#"
    SELECT * from card
    INNER JOIN card_has_legality
    ON card.id = card_has_legality.card_id
    INNER JOIN legality
    ON legality.id = card_has_legality.legality_id
    INNER JOIN format
    ON legality.format_id = format.id
    WHERE format.name = ?1
    AND legality.status
    IN (?2)
    ORDER BY card.name
"#;

pub const FETCH_CARDS_BY_DECK_ID_STMT: &str = r#"
    SELECT * FROM card
    INNER JOIN deck_has_card
    ON card.id = deck_has_card.card_id
    INNER JOIN decks
    ON decks.id = deck_has_card.deck_id
    WHERE decks.id = ?1
"#;

pub const DELETE_CARD_BY_ID_STMT: &str = r#"
    DELETE FROM card WHERE id = ?
"#;

/// Returns `1` if a card exists in the database with specified unique quad (`set_code`,
/// `language`, `number`, `is_foil`)`fields. Returns `0` otherwise.
pub const COUNT_ONE_CARD_BY_UNIQUE_QUAD_STMT: &str = r#"
    SELECT COUNT(1) from card
    WHERE UPPER(set_code) = UPPER(?1)
    AND UPPER(language) = UPPER(?2)
    AND number = ?3
    AND is_foil = ?4
"#;
