let project_name = variable::get("project-name");

file::rename("proto/service.proto", `proto/${to_snake_case(project_name)}.proto`);
file::rename("src/service.rs", `src/${to_snake_case(project_name)}_service.rs`);
