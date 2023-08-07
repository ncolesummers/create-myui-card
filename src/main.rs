use clap::Parser;
use include_dir::{include_dir, Dir};
use std::io::{self, Write};
use std::process::Command;

/// Simple Program to scaffold an Experience Extension
#[derive(Parser)]
struct Cli {
    /// The name of the project
    project: String,
}

static TEMPLATES: Dir<'_> = include_dir!("./template");

/// Get the skeleton of the project. This will be the base of the project. Provided by Ellucian.
fn get_skeleton(project: &str) {
    let args = vec!["--yes", "-p", "https://cdn.elluciancloud.com/assets/SDK/latest/ellucian-create-experience-extension-latest.tgz", "create-experience-extension", &project];
    let command = Command::new("npx")
        .args(args)
        .output()
        .expect("initial project creation failed");
    io::stdout().write_all(&command.stdout).unwrap();
}

fn json_package_set(key: &str, value: &str, project: &str) {
    println!("{}: {}", key, value);
    let project_dir = format!("./{}", &project);
    let command = Command::new("npm")
        .current_dir(project_dir)
        .arg("pkg")
        .arg("set")
        .arg(format!("{}={}", key, value))
        .output()
        .expect(format!("failed to update package.json for {}", key).as_str());
    io::stdout().write_all(&command.stdout).unwrap();
}

/// Install the dependencies for the project
fn install_dependencies(dependencies: Vec<&str>, save_option: &str, project: &str) {
    let project_dir = format!("./{}", &project);
    let command = Command::new("npm")
        .current_dir(project_dir)
        .arg("install")
        .arg(save_option)
        .args(dependencies)
        .output()
        .expect("dependency installation failed");

    io::stdout().write_all(&command.stdout).unwrap();
    io::stderr().write_all(&command.stderr).unwrap();
}

fn main() {
    let args = Cli::parse();
    let project = args.project;
    get_skeleton(&project);

    let runtime_dependencies = vec!["@tanstack/react-query"];
    install_dependencies(runtime_dependencies, "-P", &project);
    let dev_dependencies = vec![
        "@storybook/addon-a11y",
        "@storybook/addon-essentials",
        "@storybook/addon-interactions",
        "@storybook/addon-links",
        "@storybook/addon-onboarding",
        "@storybook/blocks",
        "@storybook/jest",
        "@storybook/react",
        "@storybook/react-webpack5",
        "@storybook/test-runner",
        "@storybook/testing-library",
        "storybook",
        "axe-playwright",
        "eslint-config-prettier",
        "eslint-plugin-react-hooks",
        "eslint-plugin-react-refresh",
        "eslint-plugin-storybook",
        "msw",
        "msw-storybook-addon",
    ];
    install_dependencies(dev_dependencies, "-D", &project);

    let exact_dependencies = vec!["prettier"];
    install_dependencies(exact_dependencies, "-E", &project);

    // extract the template files into the project
    TEMPLATES.extract(&project).unwrap();

    // update the publisher in extension.js from Sample to University of Idaho
    let extension_config_file = format!("./{}/extension.js", &project);
    let extension_config = std::fs::read_to_string(&extension_config_file)
        .unwrap()
        .replace("Sample", "University of Idaho");
    std::fs::write(extension_config_file, extension_config).unwrap();

    // get the required node version from the .nvmrc file
    let node_version = std::fs::read_to_string(format!("./{}/.nvmrc", &project)).unwrap();

    // build hashmap of package.json keys and values
    // that need to be updated
    let mut package_json_updates = std::collections::HashMap::new();
    package_json_updates.insert(
        "scripts.format",
        "prettier --write \"**/*.{js,jsx,ts,tsx,css,md,json,yml,yaml}\"",
    );
    package_json_updates.insert("scripts.prepare", "husky install");
    package_json_updates.insert("scripts.build-storybook", "storybook build");
    package_json_updates.insert("scripts.storybook", "storybook dev -p 6006");
    package_json_updates.insert("scripts.test-storybook", "test-storybook");
    package_json_updates.insert(
        "setup-tests",
        "npx --yes playwright install --with-deps && npm run build-storybook --quiet",
    );
    package_json_updates.insert(
        "scripts.test",
        "npx --yes concurrently -k -s first -n \"SB,TEST\" -c \"magenta,blue\" \"npx http-server storybook-static --port 6006 --silent\" \"npx wait-on tcp:6006 && yarn test-storybook\""
    );
    package_json_updates.insert("volta.node", node_version.trim());

    // update the package.json file
    for (key, value) in package_json_updates {
        json_package_set(key, value, &project);
    }
}
