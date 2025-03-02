# Contributing guidelines

Ways to contribute:

- Bug reports
- Source code
- Documentation
- Feedback

## Bug reports

- Open reports for specific issues; do not use a single issue for multiple bugs
- Follow the comments in the issue template
- Do not delete fields in the issue template, even if unused
- Keep discussions on topic

## Pull requests

- Develop in a branch on your fork
- Each commit must be signed off, using real names and addresses
- Each commit must accurately describe the change

## Code

New code or modifications to existing code shall be made available under the
terms of the [BSD-2-Clause Plus Patent][BSD-2-Clause-Patent] license.

```
SPDX-License-Identifier: BSD-2-Clause-Patent
```

- Use `cargo +nightly fmt` to format the code
- Use all lowercase for file names
- Do not use spaces in file names
- Prefer underscores (`_`) over dashes (`-`) in file names
- Prefer dashes (`-`) over underscores (`_`) in package names

### Configuration files

Configuration data may instead be made available under the terms of the
[CC0 1.0 Universal Public Domain Dedication][CC0-1.0].

```
SPDX-License-Identifier: CC0-1.0
```

## Documentation

New code or modifications to existing code shall be made available under the
terms of the [BSD-2-Clause Plus Patent][BSD-2-Clause-Patent] license.

```
SPDX-License-Identifier: BSD-2-Clause-Patent
```

- Use American English for spelling
- Use a serial comma in lists
- Use sentence case for titles and section headers
- Use all lowercase for file names
- Do not use spaces in file names
- Prefer dashes (`-`) over underscores (`_`) in file names
- Place all links at the bottom of the document, separated by 2 blank lines


[BSD-2-Clause-Patent]: https://spdx.org/licenses/BSD-2-Clause-Patent.html
[CC0-1.0]: https://creativecommons.org/publicdomain/zero/1.0/legalcode
