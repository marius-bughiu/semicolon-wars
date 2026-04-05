# Factions

> Before you open a PR, declare a faction. A faction is a stylistic worldview you are willing to fight for. You may switch factions between PRs, but never mid-thread — switching mid-thread is known as *apostasy* and is the only recognized form of disgrace in the arena.

This list is not exhaustive. To add a new faction, open an issue using the `new_faction` template.

---

## The Semicolonists

- **Creed:** Every statement ends with a semicolon. To omit one is to leave a sentence without a period.
- **Enemies:** The Whitespace Minimalists, ASI Truthers.
- **Sacred text:** ECMA-262, §Automatic Semicolon Insertion — cited ironically, as proof of why the feature should have never existed.
- **Slogan:** *The grammar is not a suggestion.*

## The Whitespace Minimalists

- **Creed:** If the parser does not require it, do not type it. Semicolons, parentheses around single arguments, trailing commas — all unwelcome.
- **Enemies:** The Semicolonists, House of Trailing Comma.
- **Sacred text:** The Python `import this` output, lines 2 and 3.
- **Slogan:** *Sparse is better than dense.*

## The Tabbed Brotherhood

- **Creed:** Tabs are semantic. Spaces are a pixel-level lie told by people who cannot configure their editors.
- **Enemies:** The Space Cadets, any editorconfig that sets `indent_style = space`.
- **Sacred text:** The Linux kernel coding style, §1.
- **Slogan:** *One character, many widths, perfect freedom.*

## The Space Cadets

- **Creed:** Spaces render identically everywhere. That is the entire argument and it is sufficient.
- **Enemies:** The Tabbed Brotherhood.
- **Sacred text:** PEP 8, §Indentation.
- **Slogan:** *Determinism over preference.*

## Order of gofmt

- **Creed:** There is one correct formatting, it is whatever `gofmt` emits, and the debate is closed. To debate it is to misunderstand Go.
- **Enemies:** Anyone who has ever opened a style PR against a `.go` file.
- **Sacred text:** `gofmt -d` output on a dirty file.
- **Slogan:** *The tool is the argument.*

## The Early Returners

- **Creed:** Guard clauses and early returns. Nested conditionals are a failure of nerve.
- **Enemies:** The Single-Exit Loyalists.
- **Sacred text:** "Replace Nested Conditional with Guard Clauses," Fowler, *Refactoring*, 1999.
- **Slogan:** *Return before you nest.*

## The Single-Exit Loyalists

- **Creed:** A function has one entry and one exit. This is structured programming. Everything else is cowboy code.
- **Enemies:** The Early Returners.
- **Sacred text:** Dijkstra, *A Discipline of Programming*.
- **Slogan:** *One door in, one door out.*

## House of Trailing Comma

- **Creed:** A trailing comma on the last element of a list or object is not noise — it is preparation for the next line, which may arrive at any moment.
- **Enemies:** The Whitespace Minimalists.
- **Sacred text:** Any well-formed `package.json`.
- **Slogan:** *Always leave the list open.*

## Snake_case Loyalists

- **Creed:** Words are separated by underscores. Words are not separated by capital letters, because capital letters are for proper nouns and sentence beginnings, not variable names.
- **Enemies:** The Camelists, The Pascalers.
- **Sacred text:** PEP 8, §Naming Conventions.
- **Slogan:** *Underscore or perish.*

## The Camelists

- **Creed:** `camelCase` for variables, `PascalCase` for types. This is how every major ecosystem does it and to do otherwise is provincialism.
- **Enemies:** Snake_case Loyalists.
- **Sacred text:** The JavaScript standard library.
- **Slogan:** *The hump is load-bearing.*

## The Pascalers

- **Creed:** `PascalCase` for everything. The distinction between a function and a type is for the compiler, not for the reader.
- **Enemies:** Everyone. It is lonely being a Pascaler.
- **Sacred text:** Turbo Pascal 7.0 documentation.
- **Slogan:** *Capitalize with intent.*

## ASI Truthers

- **Creed:** Automatic Semicolon Insertion is a feature of the language, and refusing to use it is a refusal to learn the language.
- **Enemies:** The Semicolonists.
- **Sacred text:** "JavaScript Semicolon Insertion" by Isaac Z. Schlueter, 2010.
- **Slogan:** *Trust the parser.*

## The Unformatted

- **Creed:** All formatters are tyranny. Every line is a personal decision. `prettier`, `black`, `gofmt`, and `rustfmt` are all the same enemy under different hats.
- **Enemies:** Order of gofmt, and most of the modern tooling ecosystem.
- **Sacred text:** Any uncommitted working directory.
- **Slogan:** *Let my diffs be long.*
