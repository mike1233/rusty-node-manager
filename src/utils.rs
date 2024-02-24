pub mod node_utils {
    use regex::Regex;

    pub fn node_version_compare_fn(a: &str, b: &str) -> std::cmp::Ordering {
        let re = Regex::new(r"[^\d.]").unwrap();

        let a = re.replace_all(a, "").to_string();
        let b = re.replace_all(b, "").to_string();
        // result of the regex would look something like: "14.17.0"

        let a_num: Vec<&str> = a.split(".").collect();
        let b_num: Vec<&str> = b.split(".").collect();

        let a_major = a_num[0].parse::<i32>().unwrap();
        let b_major = b_num[0].parse::<i32>().unwrap();

        let a_minor = a_num[1].parse::<i32>().unwrap();
        let b_minor = b_num[1].parse::<i32>().unwrap();

        let a_patch = a_num[2].parse::<i32>().unwrap();
        let b_patch = b_num[2].parse::<i32>().unwrap();

        if a_major > b_major {
            std::cmp::Ordering::Greater
        } else if a_major < b_major {
            std::cmp::Ordering::Less
        } else {
            if a_minor > b_minor {
                std::cmp::Ordering::Greater
            } else if a_minor < b_minor {
                std::cmp::Ordering::Less
            } else {
                if a_patch > b_patch {
                    std::cmp::Ordering::Greater
                } else if a_patch < b_patch {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        }
    }

    pub fn create_node_windows_download_url(base_node_url: &str, version: &str) -> String {
        let download_url: String =
            base_node_url.to_string() + version + "/node-" + version + "-win-x64.zip";
        download_url
    }
}
