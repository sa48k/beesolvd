# beesolved v0.1

A solver for the NY Times' Spelling Bee puzzle, written in Rust. Provide the centre letter and the six other letters as an argument, e.g.

    `./beesolved rinbsna`

This will find words that are four letters or longer, and can be spelled using some or all of these letters, always including the first letter (*r* in this example). Results will be displayed in increasing order of lengths, i.e. starting with four-letter words, ending with long words/pangrams.