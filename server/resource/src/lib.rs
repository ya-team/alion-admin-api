use anyhow::Result;
use askama::Template;
use std::path::PathBuf;

#[allow(dead_code)]
mod filters {
    #[allow(dead_code)]
    pub fn lower(s: &str) -> ::askama::Result<String> {
        Ok(s.to_lowercase())
    }

    #[allow(dead_code)]
    pub fn title(s: &str) -> ::askama::Result<String> {
        Ok(s.to_string())
    }
}

#[derive(Template)]
#[template(path = "error.rs.askama", escape = "none")]
struct ErrorTemplate {
    name: String,
}

impl ErrorTemplate {
    fn lower(&self) -> String {
        self.name.to_lowercase()
    }
}

#[derive(Template)]
#[template(path = "input.rs.askama", escape = "none")]
struct InputTemplate {
    name: String,
}

impl InputTemplate {
    fn lower(&self) -> String {
        self.name.to_lowercase()
    }
}

#[derive(Template)]
#[template(path = "service.rs.askama", escape = "none")]
struct ServiceTemplate {
    name: String,
}

impl ServiceTemplate {
    fn lower(&self) -> String {
        self.name.to_lowercase()
    }
}

#[derive(Template)]
#[template(path = "api.rs.askama", escape = "none")]
struct ApiTemplate {
    name: String,
}

impl ApiTemplate {
    fn lower(&self) -> String {
        self.name.to_lowercase()
    }

    fn title(&self) -> String {
        self.name.to_string()
    }
}

pub fn generate_code(name: &str, base_path: impl Into<PathBuf>) -> Result<()> {
    let base_path = base_path.into();
    let name = name.to_string();

    let error_template = ErrorTemplate { name: name.clone() };
    let error_code = error_template.render()?;
    let error_path = base_path
        .join("server")
        .join("service")
        .join("src")
        .join("admin")
        .join("errors")
        .join(format!("sys_{}_error.rs", name.to_lowercase()));
    std::fs::create_dir_all(error_path.parent().unwrap())?;
    std::fs::write(error_path, error_code)?;

    let input_template = InputTemplate { name: name.clone() };
    let input_code = input_template.render()?;
    let input_path = base_path
        .join("server")
        .join("model")
        .join("src")
        .join("admin")
        .join("input")
        .join(format!("sys_{}.rs", name.to_lowercase()));
    std::fs::create_dir_all(input_path.parent().unwrap())?;
    std::fs::write(input_path, input_code)?;

    let service_template = ServiceTemplate { name: name.clone() };
    let service_code = service_template.render()?;
    let service_path = base_path
        .join("server")
        .join("service")
        .join("src")
        .join("admin")
        .join(format!("sys_{}_service.rs", name.to_lowercase()));
    std::fs::create_dir_all(service_path.parent().unwrap())?;
    std::fs::write(service_path, service_code)?;

    let api_template = ApiTemplate { name: name.clone() };
    let api_code = api_template.render()?;
    let api_path = base_path
        .join("server")
        .join("api")
        .join("src")
        .join("admin")
        .join(format!("sys_{}_api.rs", name.to_lowercase()));
    std::fs::create_dir_all(api_path.parent().unwrap())?;
    std::fs::write(api_path, api_code)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_code_generation() -> Result<()> {
        let current_dir = std::env::current_dir()?;
        println!("当前目录: {:?}", current_dir);

        generate_code("Role", &current_dir)?;

        let files_to_check = vec![
            current_dir
                .join("server")
                .join("service")
                .join("src")
                .join("admin")
                .join("errors")
                .join("sys_role_error.rs"),
            current_dir
                .join("server")
                .join("model")
                .join("src")
                .join("admin")
                .join("input")
                .join("sys_role.rs"),
            current_dir
                .join("server")
                .join("service")
                .join("src")
                .join("admin")
                .join("sys_role_service.rs"),
            current_dir
                .join("server")
                .join("api")
                .join("src")
                .join("admin")
                .join("sys_role_api.rs"),
        ];

        println!("检查以下文件:");
        for file in &files_to_check {
            println!("  - {:?}", file);
        }

        for file in files_to_check {
            if !file.exists() {
                println!("文件不存在: {:?}", file);
                assert!(file.exists(), "File {:?} should exist", file);
            }
            let content = fs::read_to_string(&file)?;
            assert!(!content.is_empty(), "File {:?} should not be empty", file);

            assert!(
                content.contains("Role"),
                "File {:?} should contain 'Role'",
                file
            );

            if file.ends_with("sys_role_error.rs") {
                assert!(content.contains("RoleError"));
                assert!(content.contains("RoleNotFound"));
            } else if file.ends_with("sys_role.rs") {
                assert!(content.contains("RolePageRequest"));
                assert!(content.contains("RoleInput"));
            } else if file.ends_with("sys_role_service.rs") {
                assert!(content.contains("RoleService"));
                assert!(content.contains("TRoleService"));
            } else if file.ends_with("sys_role_api.rs") {
                assert!(content.contains("RoleApi"));
                assert!(content.contains("get_role"));
            }
        }

        Ok(())
    }
}
