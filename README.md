# m3u_spotify
imports an m3u into spotify using idv3 data

very simple 1 off rust program that has hardcoded values to convert a m3u file to a spotify playlist

process
* creates new playlist
* for each file in a .m3u
* * read the idv3 tags
* * search spotify for `artist` & `track`
* * adds track to playlist
