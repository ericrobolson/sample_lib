# sample_lib
A sample library manager.

User dumps files into 'samples', and the `sample_lib` will go through and categorize them through a TUI. 


## Technical Features
Each processed file is hashed through MD5 and stored. Any duplicates are skipped and deleted.

Uses `hound` or `rodio` to play sample files for previewing

Define data types in line separated files or CSVs and nested folders. E.g. a `instrument-types.txt` has contents with
```
instrument,abbreviation
drums/kicks,kd
drums/high-hats/closed,hh
drums/high-hats/open,oh
drums/snares,sn
```

# Roadmap
- [ ] Container for generic grids
- [ ] Project structure generation
- [ ] Searches source directory for unhandles samples, then has user categorize them. Outputs 