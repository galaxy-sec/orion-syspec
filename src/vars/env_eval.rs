use std::env;

use log::debug;
use tracing::error;

pub fn expand_env_vars(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' && chars.peek() == Some(&'{') {
            // 跳过 '{'
            chars.next();

            // 收集变量名
            let mut var_name = String::new();
            let mut found_closing_brace = false;

            for c in chars.by_ref() {
                if c == '}' {
                    found_closing_brace = true;
                    break;
                }
                var_name.push(c);
            }

            // 处理变量替换
            if found_closing_brace {
                match env::var(&var_name) {
                    Ok(value) => {
                        debug!("get env var {} : {}", var_name, value);
                        result.push_str(&value);
                    }
                    Err(_) => {
                        error!("not get env var :{}", var_name);
                        result.push_str("${");
                        result.push_str(&var_name);
                        result.push('}');
                    }
                }
            } else {
                // 未闭合的花括号
                result.push_str("${");
                result.push_str(&var_name);
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{tools::get_repo_name, vars::env_eval::expand_env_vars};

    #[test]
    fn test_get_last_segment() {
        // 测试HTTP URL
        assert_eq!(
            get_repo_name("https://github.com/user/repo.git"),
            Some("repo.git".to_string())
        );

        // 测试HTTPS URL
        assert_eq!(
            get_repo_name("https://github.com/user/repo"),
            Some("repo".to_string())
        );

        // 测试SSH格式Git地址
        assert_eq!(
            get_repo_name("git@github.com:user/repo.git"),
            Some("repo.git".to_string())
        );

        // 测试SSH格式不带.git后缀
        assert_eq!(
            get_repo_name("git@gitlab.com:group/subgroup/repo"),
            Some("repo".to_string())
        );

        // 测试无效URL
        assert_eq!(get_repo_name("not_a_url"), None);
    }

    #[test]
    fn test_basic_expansion() {
        unsafe { env::set_var("HOME", "/home/user") };
        assert_eq!(expand_env_vars("${HOME}/bin"), "/home/user/bin");
    }

    #[test]
    fn test_multiple_variables() {
        unsafe { env::set_var("USER", "john") };
        unsafe { env::set_var("APP", "myapp") };
        assert_eq!(
            expand_env_vars("/opt/${APP}/bin/${USER}"),
            "/opt/myapp/bin/john"
        );
    }

    #[test]
    fn test_undefined_variable() {
        unsafe { env::remove_var("UNDEFINED_VAR") };
        assert_eq!(
            expand_env_vars("Path: ${UNDEFINED_VAR}/data"),
            "Path: ${UNDEFINED_VAR}/data"
        );
    }

    #[test]
    fn test_nested_braces() {
        unsafe { env::set_var("VAR", "value") };
        assert_eq!(expand_env_vars("${VAR}}"), "value}");
        assert_eq!(expand_env_vars("${VAR}}}"), "value}}");
    }

    #[test]
    fn test_unclosed_brace() {
        unsafe { env::set_var("HOME", "/home/user") };
        assert_eq!(expand_env_vars("${HOME"), "${HOME");
        assert_eq!(expand_env_vars("${HOME${USER"), "${HOME${USER");
    }

    #[test]
    fn test_empty_variable_name() {
        assert_eq!(expand_env_vars("${}"), "${}");
    }

    #[test]
    fn test_special_characters() {
        unsafe { env::set_var("VAR_WITH_UNDERSCORE", "ok") };
        assert_eq!(expand_env_vars("${VAR_WITH_UNDERSCORE}"), "ok");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(expand_env_vars(""), "");
        assert_eq!(expand_env_vars("no variables"), "no variables");
        assert_eq!(expand_env_vars("$"), "$");
        assert_eq!(expand_env_vars("${"), "${");
        assert_eq!(expand_env_vars("}"), "}");
        assert_eq!(expand_env_vars("${}"), "${}");
    }

    #[test]
    fn test_consecutive_variables() {
        unsafe { env::set_var("A", "1") };
        unsafe { env::set_var("B", "2") };
        assert_eq!(expand_env_vars("${A}${B}"), "12");
    }
}
