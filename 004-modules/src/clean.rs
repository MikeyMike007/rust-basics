pub mod clean {
    pub fn perform_clean() {
        println!("Cleaning");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}
