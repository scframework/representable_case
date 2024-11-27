use representable_macro::Representable;

#[derive(Representable)]
struct Os {
    name: String,
    age: u32,
    distros: Vec<String>,
}

#[derive(Representable)]
struct Server {
    name: String,
    release_year: i32,
    current_version: String,
    os_support: Vec<Os>,
}


#[cfg(test)]
mod tests {
    use crate::{Os, Server};
    use representable_interface::Representable;

    pub fn linux() -> Os {
        Os {
            name: "linux".to_string(),
            age: 33,
            distros: vec![
                "ubuntu".to_string(),
                "debian".to_string(),
                "arch linux".to_string(),
                "kali linux".to_string(),
            ],
        }
    }

    #[test]
    pub fn test_macro() {
        let linux: Os = linux();
        let representation_str = r#"Os { name: linux, age: 33, distros: ["ubuntu", "debian", "arch linux", "kali linux"] }"#.to_string();
        assert_eq!(representation_str, linux.represent())
    }

    #[test]
    pub fn test_macro_array_of_os() {
        let apache = Server {
            name: "apache".to_string(),
            release_year: 1995,
            current_version: "2.4.62".to_string(),
            os_support: vec![linux()]
        };
        let representation_str = format!(
            r#"Server {{ name: apache, release_year: 1995, current_version: 2.4.62, os_support: [{}] }}"#,
            linux().represent()
        );
        assert_eq!(representation_str, apache.represent());
    }

}
