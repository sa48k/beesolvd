# beesolved v0.1

A solver for the NY Times' Spelling Bee puzzle. Provide the centre letter and the six other letters, e.g.

    `./beesolved r inbsna`

This will find words that are four letters or longer, and can be spelled using some or all of these letters, always including the first letter (*r* in this example). Results will be displayed in decreasing order of lengths, i.e. long words/pangrams first, and ending with four-letter words.