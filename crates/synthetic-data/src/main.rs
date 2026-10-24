use anyhow::Result;
use chrono::Utc;
use rand::Rng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

const SEED: u64 = 42;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub seed: Option<u64>,
    pub record_count: Option<usize>,
    pub schemas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermission {
    pub role_id: String,
    pub permission_id: String,
}

#[derive(Debug)]
struct SyntheticGenerator {
    rng: rand::rngs::StdRng,
    seed: u64,
    record_count: usize,
    schemas: Vec<String>,
    users: Vec<User>,
    roles: Vec<Role>,
    permissions: Vec<Permission>,
    user_roles: Vec<UserRole>,
    role_permissions: Vec<RolePermission>,
}

impl SyntheticGenerator {
    fn new(config: &GeneratorConfig) -> Self {
        let seed = config.seed.unwrap_or(SEED);
        let rng = rand::rngs::StdRng::seed_from_u64(seed);
        let record_count = config.record_count.unwrap_or(100);
        let schemas = config.schemas.clone();

        SyntheticGenerator {
            rng,
            seed,
            record_count,
            schemas,
            users: Vec::new(),
            roles: Vec::new(),
            permissions: Vec::new(),
            user_roles: Vec::new(),
            role_permissions: Vec::new(),
        }
    }

    fn generate_usernames(&mut self) -> Vec<String> {
        let adjectives = [
            "quick", "lazy", "happy", "sad", "fast", "slow", "bright", "dark", "soft", "hard",
        ];
        let nouns = [
            "user", "customer", "account", "profile", "profile", "member", "client", "account",
            "accounting", "contact",
        ];

        (0..self.record_count)
            .map(|i| {
                let adj = adjectives.get(self.rng.gen_range(0..adjectives.len())).unwrap_or(&"user");
                let noun = nouns.get(self.rng.gen_range(0..nouns.len())).unwrap_or(&"user");
                format!(
                    "{}{}",
                    adj,
                    noun.to_string() + &i.to_string().split('').last().unwrap_or("0")
                )
            })
            .collect()
    }

    fn generate_emails(&self, usernames: &[String]) -> Vec<String> {
        usernames
            .iter()
            .map(|u| format!("{}@telecom.local", u))
            .collect()
    }

    fn generate_password_hash() -> String {
        format!(
            "$6${}",
            rand::thread_rng()
                .sample_iter(rand::distributions::Alphanumeric)
                .take(16)
                .map(char::from)
                .collect::<String>()
        )
    }

    fn generate_users(&mut self, usernames: &[String]) -> Vec<User> {
        usernames
            .iter()
            .enumerate()
            .map(|(i, username)| {
                let email = format!("{}@telecom.local", username);
                let password_hash = Self::generate_password_hash();
                let now = Utc::now().to_rfc3339();
                User {
                    id: uuid::Uuid::new_v4().to_string(),
                    username: username.clone(),
                    email,
                    password_hash,
                    created_at: now.clone(),
                    updated_at: now,
                }
            })
            .collect()
    }

    fn generate_roles(&mut self) -> Vec<Role> {
        let roles = [
            ("admin", "System administrator", Some("Has full access to all resources")),
            ("user", "Standard user", Some("Limited access to own data")),
            ("viewer", "Read-only viewer", Some("Can view but not modify data")),
        ];

        roles
            .iter()
            .enumerate()
            .map(|(i, (name, desc, _))| {
                let id = uuid::Uuid::new_v4().to_string();
                let now = Utc::now().to_rfc3339();
                Role {
                    id,
                    name: name.to_string(),
                    description: desc.map(|d| d.to_string()),
                    created_at: now.clone(),
                }
            })
            .collect()
    }

    fn generate_permissions(&self, roles: &[Role]) -> Vec<Permission> {
        let perms = [
            ("manage_users", "user", "CRUD"),
            ("view_dashboard", "dashboard", "READ"),
            ("create_ticket", "ticket", "CREATE"),
            ("resolve_ticket", "ticket", "RESOLVE"),
            ("manage_products", "product", "CRUD"),
            ("view_reports", "report", "READ"),
        ];

        perms
            .iter()
            .enumerate()
            .map(|(i, (name, resource, action))| {
                let id = uuid::Uuid::new_v4().to_string();
                let now = Utc::now().to_rfc3339();
                Permission {
                    id,
                    name: name.to_string(),
                    resource: resource.to_string(),
                    action: action.to_string(),
                    created_at: now,
                }
            })
            .collect()
    }

    fn generate_user_roles(&self, users: &[User], roles: &[Role]) -> Vec<UserRole> {
        let mut result = Vec::new();
        let admin_role = &roles[0]; // admin is always first
        let user_role = &roles[1]; // user is second

        // Assign admin role to first user
        result.push(UserRole {
            user_id: users[0].id.clone(),
            role_id: admin_role.id.clone(),
        });

        // Assign user role to remaining users
        for user in &users[1..] {
            result.push(UserRole {
                user_id: user.id.clone(),
                role_id: user_role.id.clone(),
            });
        }

        result
    }

    fn generate_role_permissions(&self, roles: &[Role], permissions: &[Permission]) -> Vec<RolePermission> {
        let mut result = Vec::new();

        // Admin gets all permissions
        for perm in permissions {
            result.push(RolePermission {
                role_id: roles[0].id.clone(),
                permission_id: perm.id.clone(),
            });
        }

        // User gets basic permissions
        let user_perm_ids = permissions
            .iter()
            .take(3)
            .map(|p| p.id.clone())
            .collect::<Vec<_>>();

        for _ in &roles[1..] {
            for perm_id in &user_perm_ids {
                result.push(RolePermission {
                    role_id: roles[1].id.clone(),
                    permission_id: perm_id.clone(),
                });
            }
        }

        result
    }

    fn generate_all(&mut self) -> HashMap<String, Vec<u8>> {
        let mut result = HashMap::new();

        for schema in &self.schemas {
            let users = self.generate_users(&self.generate_usernames());
            let roles = self.generate_roles();
            let perms = self.generate_permissions(&roles);
            let user_roles = self.generate_user_roles(&users, &roles);
            let role_perms = self.generate_role_permissions(&roles, &perms);

            self.users = users;
            self.roles = roles;
            self.permissions = perms;
            self.user_roles = user_roles;
            self.role_permissions = role_perms;

            let data = serde_json::to_string(&serde_json::json({
                "seed": self.seed,
                "record_count": self.record_count,
                "schema": schema,
                "users": self.users,
                "roles": self.roles,
                "permissions": self.permissions,
                "user_roles": self.user_roles,
                "role_permissions": self.role_permissions,
            })).unwrap();

            result.insert(schema.clone(), data.as_bytes().to_vec());
        }

        result
    }

    pub async fn run(&mut self, connection_pool: &sqlx::PgPool) -> Result<()> {
        info!("Starting synthetic data generation (seed={}, records={})", self.seed, self.record_count);

        let generated = self.generate_all();

        for (schema, data) in &generated {
            info!("Generating data for schema: {}", schema);

            // Parse the generated data and insert into PostgreSQL
            let records: serde_json::Value = serde_json::from_slice(data)?;

            if let Some(users) = records.get("users").and_then(|u| u.as_array()) {
                for user in users {
                    if let Some(user_obj) = user.as_object() {
                        let sql = format!(
                            "INSERT INTO {}.users (id, username, email, password_hash, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', NOW(), NOW())",
                            schema,
                            user_obj.get("username").and_then(|v| v.as_str()).unwrap_or(""),
                            user_obj.get("email").and_then(|v| v.as_str()).unwrap_or(""),
                            user_obj.get("password_hash").and_then(|v| v.as_str()).unwrap_or(""),
                        );
                        sqlx::query(&sql).execute(connection_pool).await?;
                    }
                }
            }

            if let Some(roles) = records.get("roles").and_then(|r| r.as_array()) {
                for role in roles {
                    if let Some(role_obj) = role.as_object() {
                        let sql = format!(
                            "INSERT INTO {}.roles (id, name, description, created_at) VALUES ('{}', '{}', '{}', NOW())",
                            schema,
                            role_obj.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                            role_obj
                                .get("description")
                                .and_then(|v| v.as_str())
                                .unwrap_or(""),
                        );
                        sqlx::query(&sql).execute(connection_pool).await?;
                    }
                }
            }

            if let Some(perms) = records.get("permissions").and_then(|p| p.as_array()) {
                for perm in perms {
                    if let Some(perm_obj) = perm.as_object() {
                        let sql = format!(
                            "INSERT INTO {}.permissions (id, name, resource, action, created_at) VALUES ('{}', '{}', '{}', '{}', NOW())",
                            schema,
                            perm_obj.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                            perm_obj.get("resource").and_then(|v| v.as_str()).unwrap_or(""),
                            perm_obj.get("action").and_then(|v| v.as_str()).unwrap_or(""),
                        );
                        sqlx::query(&sql).execute(connection_pool).await?;
                    }
                }
            }

            if let Some(ur) = records.get("user_roles").and_then(|u| u.as_array()) {
                for ur_obj in ur {
                    if let Some(ur_obj) = ur_obj.as_object() {
                        let sql = format!(
                            "INSERT INTO {}.user_roles (user_id, role_id) VALUES ('{}', '{}')",
                            schema,
                            ur_obj.get("user_id").and_then(|v| v.as_str()).unwrap_or(""),
                            ur_obj.get("role_id").and_then(|v| v.as_str()).unwrap_or(""),
                        );
                        sqlx::query(&sql).execute(connection_pool).await?;
                    }
                }
            }

            if let Some(rp) = records.get("role_permissions").and_then(|r| r.as_array()) {
                for rp_obj in rp {
                    if let Some(rp_obj) = rp_obj.as_object() {
                        let sql = format!(
                            "INSERT INTO {}.role_permissions (role_id, permission_id) VALUES ('{}', '{}')",
                            schema,
                            rp_obj.get("role_id").and_then(|v| v.as_str()).unwrap_or(""),
                            rp_obj.get("permission_id").and_then(|v| v.as_str()).unwrap_or(""),
                        );
                        sqlx::query(&sql).execute(connection_pool).await?;
                    }
                }
            }

            info!("Completed data generation for schema: {}", schema);
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_str = std::env::var("SYNTHETIC_CONFIG").unwrap_or_else(|_| {
        r#"{"seed":42,"record_count":10,"schemas":["auth","crm","billing","inventory","trouble_ticket","catalog"]}"#
        .to_string()
    });

    let config: GeneratorConfig = serde_json::from_str(&config_str)?;
    info!("Synthetic data config: {:?}", config);

    let mut generator = SyntheticGenerator::new(&config);

    // Connect to PostgreSQL
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|| {
        "postgres://postgres:postgres@localhost:5432/telecom".to_string()
    });
    let pool = sqlx::postgres::PostgreSqlPool::connect(&database_url).await?;

    generator.run(&pool).await?;

    info!("Synthetic data generation complete!");

    Ok(())
}