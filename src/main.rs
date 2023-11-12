use project_tree_manager::cli;
use project_tree_manager::generate;
use project_tree_manager::update;

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let config = &sub_matches
                .value_of_t::<String>("config")
                .unwrap_or_else(|e| e.exit());
            let folder = &sub_matches
                .value_of_t::<String>("folder")
                .unwrap_or_else(|e| e.exit());
            generate(config, folder)
        }
        Some(("update", sub_matches)) => {
            let force = sub_matches.get_flag("force");
            update(force)
        }
        _ => unreachable!(),
    }
}
