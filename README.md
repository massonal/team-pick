# Cheesy Chairman Chooser

This repo contains code for a team picker.
The original goal was to choose the chairman of a weekly meeting, but that can be adapted ;)

The rules for the team picker were:
- the choice has to be eventually fair (not "It's always Joe")
- no one can be chairman twice in a row

The chosen solution was to give each team member stakes, and to increment the stakes on each draw.
The chairman's stakes are reset after is is chosen, so that their odds on the next draw will be zero.

The code has been written using Github Copilot, during a trial session of this tool.
The two implementations should be working - the Python one has been tested and used,
the Rust one has not, but looks plausible (be aware that  t needs to go in a proper Rust project structure before anything).
