use super::*;
use std::env;

mod config_test {
    use super::*;
    #[test]
    fn config_param_exceed() {
        let args = vec![String::from("cmd")];
        assert_eq!(Config::new(&args).err(), Some("not enough arguments"));
    }
}

mod get_case_sensitive_test {
    use super::*;
    #[test]
    fn get_case_sensitive_default() {
        let key = "CASE_INSENSITIVE";
        env::remove_var(key);
        assert_eq!(get_case_sensitive_cfg(), false);
    }

    // TODO: need xUnit framework like:
    // https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
    #[test]
    fn get_case_sensitive_true() {
        let key = "CASE_INSENSITIVE";
        env::set_var(key, "true");
        assert_eq!(get_case_sensitive_cfg(), true);
        /*
        assert_eq!(env::var(key), Ok("true".to_string()));
        */
        env::remove_var(key);
    }

    #[test]
    fn get_case_sensitive_false() {
        let key = "CASE_INSENSITIVE";
        env::set_var(key, "false");
        assert_eq!(get_case_sensitive_cfg(), false);
        /*
        assert_eq!(env::var(key), Ok("true".to_string()));
        */
        env::remove_var(key);
    }
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
