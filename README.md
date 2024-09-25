# `lsi` - The interactive TUI alternative to `ls`

## General Design Document

### Options (Top Left)
- Visual Mode (List, Tree)
- Visibility (Normal, Most, All)
- Color Mode (Default, Normal, High Contrast)
- Size (None, Bytes, Human Readable)
- Permission
- Date
- User
- Group
- Recursive Depth (Negative implies infinite)

### Display (Right)
- Uses options to determine how files are displayed
- Scroll mode possible? (j and k)
- `Enter` to open file in default editor (or viewer) or change to directory

### Other Windows (Bottom Left)
- System Fetch
- Repo Information
- File Preview

### Controls (Bottom)
- (`h`, `j`, `k`, `l`) for movement in window
- `Shift` + (`h`, `j`, `k`, `l`) for movement between windows
- `Enter` to select
- `Esc` or `q` to quit


## Final Design Thoughts
- Checkboxes for options that have Yes/No options
- Change permissions, delete, make files/directories?
- Put controls in border rather than its own box
- Let Display be able to expand or shrink to give more space

