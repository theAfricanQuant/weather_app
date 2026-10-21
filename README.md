# Rusty Weather Learning Lab

A Pythonista-friendly, step-by-step Rust course built around the weather app in the linked video. Our mission is to use the weather app as a guide to learn how Rust projects are scaffolded and organized while building a first Rust project. We will create a binary crate in this folder, grow it one idea at a time, keep a Git checkpoint for each milestone, and publish the lessons as a small website.

## Read the lessons online

After the first successful GitHub Pages deployment, put the live URL here:

**Course site:** `https://OWNER.github.io/REPOSITORY/` *(replace OWNER and REPOSITORY after we create the GitHub repo)*

Readers can open that site directly in a browser; they do not need to clone the Rust crate. The site is deployed from the course HTML by [`.github/workflows/pages.yml`](.github/workflows/pages.yml).

## Start the crate in this folder

Open Tuxedo OS Terminal and move to the project folder:

```bash
cd "/home/siseng/Documents/08_Personal/programing_languages/Rust/weather_app"
pwd
```

Confirm that `pwd` prints the path above. If there is no `Cargo.toml` yet, create the crate in this folder, then open the project in Zed:

```bash
cargo init --bin --vcs none
zed .
```

If `zed .` is not available, open Zed from the app launcher and use **File → Open Folder** to select this project. In Zed, open `src/main.rs`. Run `cargo run` from the terminal when the lesson asks you to build and launch the starter.

`cargo init` adds the package manifest and `src/main.rs` here without creating another nested folder. `--bin` starts an executable crate; `--vcs none` leaves Git setup for the deliberate repository milestone. `cargo run` compiles and runs the starter program. The course HTML explains what each step means and how it differs from the Python workflow.

## Learning and sharing milestones

1. Create and run the starter crate.
2. Build the weather app in small lessons, with Python comparisons and Rust compiler clues.
3. Initialize a Git repository for this project, inspect the staged files, and commit a checkpoint.
4. Create a public GitHub repository, connect the local repo, and push the checkpoint.
5. Set GitHub Pages' publishing source to **GitHub Actions**. The workflow publishes the course HTML after pushes to `main`.
6. Replace the placeholder course URL above with the URL shown by the successful Pages deployment.

We will take the Git steps together, one command at a time. First verify the Git root and status; do not stage files until the root is this project:

```bash
git rev-parse --show-toplevel
git status --short
```

After the crate runs and this folder has its own Git root, initialize the main branch, inspect the staged files, and make the first checkpoint:

```bash
git init -b main
git add -A
git status --short
git commit -m "Start Rust weather learning project"
```

Create an empty **public** repository on GitHub under the account we choose. Leave “Add a README” unchecked because this project already has one. Authenticate the terminal with `gh auth login`, confirm the correct account with `gh auth status`, then connect and push:

```bash
git remote add origin https://github.com/theAfricanQuant/weather_app.git
git push -u origin main
```

Replace `OWNER` and `REPOSITORY` with the real values. If `git init` reports a permission error, or if Git still identifies the parent Rustlings folder after initialization, stop before staging files and resolve the repository boundary first.

## Project layout

- `src/` — our growing Rust application.
- `Cargo.toml` — the crate manifest and dependencies.
- `index.html`, `lessons/`, `reference/`, and `assets/` — the course site.
- `.github/workflows/pages.yml` — the GitHub Actions Pages deployment.

The complete upstream app remains the reference implementation at <https://github.com/feenix100/rusty_weather>.
