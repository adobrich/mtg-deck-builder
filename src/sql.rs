const INSERT_CARD_STMT: &str = r#"
  INSERT INTO cards (
      id,
      ability,
      artist,
      cmc,
      color_identity,
      colors,
      flavor_text,
      is_foil,
      language,
      local_count,
      loyalty,
      mana_cost,
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
    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
    ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24
  )
"#;

const CREATE_DATABASE_STMT: &str = r#"
    BEGIN;
    CREATE TABLE IF NOT EXISTS card_has_legality (
        card_id INTEGER NOT NULL,
        legality_id INTEGER NOT NULL,

        FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE,
        FOREIGN KEY (legality_id) REFERENCES legality(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS legality (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        card_id INTEGER NOT NULL,
        format_id INTEGER NOT NULL,
        status TEXT NOT NULL,

        CONSTRAINT no_duplicates UNIQUE (card_id, format_id),

        FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE,
        FOREIGN KEY (format_id) REFERENCES format(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS format (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE
    );

    CREATE TABLE IF NOT EXISTS images (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        card_id INTEGER NOT NULL,
        full BLOB,

        FOREIGN KEY (card_id) REFERENCES card(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS deck_has_card (
        deck_id INTEGER NOT NULL,
        card_id INTEGER NOT NULL,

        FOREIGN KEY (deck_id) REFERENCES decks(id) ON DELETE CASCADE,
        FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS decks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        format TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS cards (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ability TEXT,
        artist TEXT,
        cmc REAL NOT NULL,
        color_identity INTEGER NOT NULL,
        colors TEXT,
        flavor_text TEXT,
        is_foil INTEGER NOT NULL,
        local_count INTEGER NOT NULL DEFAULT 0,
        loyalty TEXT,
        mana_cost TEXT,
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
