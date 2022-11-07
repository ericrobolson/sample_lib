# sample_lib
A sample library manager.

User is guided through a TUI to build up a sample library. There are some sane defaults, but the user is essentially guided through a heirarchical workflow that is folder oriented.

Example story:
* User logs in
* Program searches `sinks` and pulls new uncategorized files
* User selects one (could be done randomly)
* User is asked the type, selects `audio`
* The program asks what the next classification is. This is selected from the subfolders, such as `loops` or `one-shot`
* User selects `one-shot`
* User then selects the instrument, such as `kd` or can add a new one.
* User inputs the key, the bpm and the note. All of these are optional
* Key, bpm and note metadata is written to file name and stored.

## Technical Features
Each processed file is hashed and stored. Any duplicates are skipped and deleted.

Uses `hound` or `rodio` to play sample files for previewing

# Roadmap
- [ ] Specify 'sinks' which samples will be pulled from. These are folder paths which will begin a recursive descent and copy over samples that haven't been processed yet. Ideally will use a hash to prevent duplicate comparisons.
- [ ] Add in hashing/checksum comparisons of murmur3. Maybe shard the file used for comparisons?
- [ ] Randomly generate loops
- [ ] Takes in a loop, then analyzes that loop and generates a new loop like the 'Brain' by Aphex Twin