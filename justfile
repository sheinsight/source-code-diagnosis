
# build:
#     echo "Building the project..."
#     cargo b --verbose --release 

# prerelease:
#     echo "Building the project for release..."
#     ./target/debug/tools

# dev:
#     echo "Running the project..."
#     cargo watch -x build

# qtest:
#     echo "Running tests..."
#     cargo test --workspace --exclude e2e -- --nocapture

test:
    #!/usr/bin/env bash
    echo "Running tests..."
    cargo test -- --nocapture


create name:
    #!/usr/bin/env bash
    echo "cp template {{name}}"

    for ext in rs json; do
        cp "./src/syntax/tmp/tmp.$ext" "./src/syntax/functions_n/{{name}}.$ext"
    done

    sed -i '' -e 's/TmpBrowserCompatMetadata/{{name}}BrowserCompatMetadata/g' \
            -e 's/TmpVisitor/{{name}}Visitor/g' \
            -e 's/tmp\.json/{{name}}\.json/g' \
            "./src/syntax/functions_n/{{name}}.rs"

    