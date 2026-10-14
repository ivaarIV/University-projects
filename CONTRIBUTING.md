# Contributing

Thank you for contributing to the Rust course exercises and projects repository.

## Before submitting

- Follow the instructor's assignment instructions and academic-integrity rules.
- Confirm that your work is permitted to be stored in this repository.
- Do not submit another student's work or copied code without required attribution.
- Never include API keys, passwords, access tokens, private student information, or unrelated personal files.
- Do not add hidden tests, confidential grading rubrics, or official solutions unless the course staff explicitly authorizes them.

## Recommended folder structure

Follow the course staff's required structure when one exists. Otherwise:

```text
exercises/
└── exercise-name/
    ├── README.md
    ├── Cargo.toml
    ├── Cargo.lock
    └── src/
        └── main.rs

projects/
└── project-name/
    ├── README.md
    ├── Cargo.toml
    ├── Cargo.lock
    └── src/
        └── main.rs
```

For a library crate, use `src/lib.rs` as appropriate. Keep each submission self-contained where practical.

## Submission steps

1. Use the fork/branch workflow specified by the course staff.
2. Create a descriptive branch, such as `submission/ownership-exercise`.
3. Add or update your exercise or project folder.
4. Include a README that explains the goal, prerequisites, how to build and run the code, how to run tests, and known limitations.
5. Run applicable checks:

   ```bash
   cargo fmt --all -- --check
   cargo test --workspace
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   ```

   Adapt these commands if your submission is not a Cargo workspace. Explain any checks that do not apply.
6. Open a Pull Request to `main` and complete the Pull Request template.
7. Address review comments and wait for an authorized maintainer's decision.

## Review policy

Submissions may be reviewed for correctness, clarity, idiomatic Rust, documentation, tests, and compliance with course requirements. A review or merge does not automatically imply a grade unless the instructor has explicitly established that policy.

## Public release after the term

The repository is expected to remain private during the term. A later public release depends on course-staff approval and a review of student permissions, personal information, assignment confidentiality, and third-party licenses. Do not assume that merging a Pull Request grants permission for public distribution.

## Questions

For assignment-specific questions, use the official course communication channel: [add course forum or contact method].
