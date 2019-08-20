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
