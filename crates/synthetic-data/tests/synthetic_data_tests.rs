use synthetic_data::GeneratorConfig;
use std::fs;

#[test]
fn test_config_parsing() {
    let config_str = r#"{"seed":42,"record_count":10,"schemas":["auth","crm"]}"#;
    let config: GeneratorConfig = serde_json::from_str(config_str).expect("Failed to parse config");
    assert_eq!(config.seed, Some(42));
    assert_eq!(config.record_count, Some(10));
    assert_eq!(config.schemas, vec!["auth", "crm"]);
}

#[test]
fn test_generator_creates_users() {
    let config = GeneratorConfig {
        seed: Some(42),
        record_count: Some(5),
        schemas: vec!["auth".to_string()],
    };

    let mut gen = synthetic_data::SyntheticGenerator::new(&config);
    let usernames = gen.generate_usernames();
    assert_eq!(usernames.len(), 5);

    let users = gen.generate_users(&usernames);
    assert_eq!(users.len(), 5);
    assert!(users.iter().all(|u| !u.username.is_empty()));
    assert!(users.iter().all(|u| u.email.contains("@telecom.local")));
}

#[test]
fn test_generator_deterministic_seed() {
    let config1 = GeneratorConfig {
        seed: Some(42),
        record_count: Some(3),
        schemas: vec!["auth".to_string()],
    };
    let config2 = GeneratorConfig {
        seed: Some(42),
        record_count: Some(3),
        schemas: vec!["auth".to_string()],
    };

    let mut gen1 = synthetic_data::SyntheticGenerator::new(&config1);
    let mut gen2 = synthetic_data::SyntheticGenerator::new(&config2);

    let usernames1 = gen1.generate_usernames();
    let usernames2 = gen2.generate_usernames();

    assert_eq!(usernames1, usernames2, "Same seed should produce same usernames");
}

#[test]
fn test_generator_different_seeds() {
    let config1 = GeneratorConfig {
        seed: Some(42),
        record_count: Some(3),
        schemas: vec!["auth".to_string()],
    };
    let config2 = GeneratorConfig {
        seed: Some(99),
        record_count: Some(3),
        schemas: vec!["auth".to_string()],
    };

    let mut gen1 = synthetic_data::SyntheticGenerator::new(&config1);
    let mut gen2 = synthetic_data::SyntheticGenerator::new(&config2);

    let usernames1 = gen1.generate_usernames();
    let usernames2 = gen2.generate_usernames();

    assert_ne!(usernames1, usernames2, "Different seeds should produce different usernames");
}

#[test]
fn test_password_hash_format() {
    let hash = synthetic_data::SyntheticGenerator::generate_password_hash();
    assert!(hash.starts_with("$6$"), "Password hash should use SHA-512 format (start with $6$)");
    assert!(hash.split('$').count() >= 7, "Password hash should have proper crypt format");
}

#[test]
fn test_record_count_respected() {
    let config = GeneratorConfig {
        seed: Some(42),
        record_count: Some(50),
        schemas: vec!["auth".to_string()],
    };

    let mut gen = synthetic_data::SyntheticGenerator::new(&config);
    let usernames = gen.generate_usernames();
    assert_eq!(usernames.len(), 50);
}

#[test]
fn test_schema_coverage() {
    let config = GeneratorConfig {
        seed: Some(42),
        record_count: Some(5),
        schemas: vec!["auth".to_string(), "crm".to_string(), "billing".to_string()],
    };

    let mut gen = synthetic_data::SyntheticGenerator::new(&config);
    assert_eq!(gen.schemas.len(), 3);
    assert!(gen.schemas.contains(&"auth".to_string()));
    assert!(gen.schemas.contains(&"crm".to_string()));
    assert!(gen.schemas.contains(&"billing".to_string()));
}