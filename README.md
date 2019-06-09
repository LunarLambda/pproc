# pproc: simple parallel file processor

`pproc` was written as an excercise of using threads in Rust.  
It is (currently) in no way to be used for serious purposes.

## Usage

`pproc <filelist> <command> [nthreads]`

`<filelist>` is a file containing a list of filenames, one per line.  
`<command>` is the command template to run. It is run through `sh -c`, so the usual quoting rules apply.  
A '%' will be substituted for the filename.  
`[nthreads]` is the number of threads to run. If omitted, it defaults to 1.  
It's a good idea to use the output of `nproc`.

The error handling is (currently) very rudimentary.

## Example

Say I wanted to encode a bunch of FLAC files to MP3 using `lame`:

You can get a list of files with `find`...  
`find ~/music/Kobaryo -name \*.flac > kobaryo.txt`

And run `lame` in parallel on (on my machine) 4 threads:  
`pproc kobaryo.txt 'lame --preset insane "%"' $(nproc)`

The output will (currently) look something like this:

```
[Thread 0] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo feat. DJ Genki - Volatile Memory.flac"
[Thread 1] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - HUG AND KILL.flac"
[Thread 2] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo feat. USAO - Sulyvahn.flac"
[Thread 3] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Eternal Ending.flac"
[Thread 1] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Delete The World.flac"
[Thread 3] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Galaxy Friends.flac"
[Thread 2] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Chocolate Lily.flac"
[Thread 0] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Eternal Ending (kyou1110 Remix).flac"
[Thread 2] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Blackbird.flac"
[Thread 1] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo feat. かめりあ - Villain Virus.flac"
[Thread 0] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Tool-Assisted Speedcore (TQBF Frame Advance RMX).flac"
[Thread 3] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Recollection.flac"
[Thread 2] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo feat. Srezcat - Speed Complexxx.flac"
[Thread 3] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - My Music Playlist.flac"
[Thread 1] lame --preset insane "/home/luna/music/Kobaryo/MIXPEED ACTION/Kobaryo - Galaxy Friends (tpz Overheat Remix).flac"
[Thread 0] lame --preset insane "/home/luna/music/Kobaryo/Kobaryo - M-e.flac"
[Thread 2] lame --preset insane "/home/luna/music/Kobaryo/Kobaryo - Glitched Character.flac"
```

## License: [MIT](LICENSE)
