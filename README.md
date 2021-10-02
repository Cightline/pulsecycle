# pulsecycle
A tool written in Rust to switch your audio outputs on Linux. This is a rewrite of [switch_stream](https://github.com/Cightline/switch_stream) in Rust. This was hastily written after getting tired of Python. 

#### There are 3 main command line options right now. 

 ```pulsecycle -c```

 This will cycle to the next available pulseaudio sink moving all streams along with it.
 
`pulsecycle -s`

 This will show the current sink in short format. (ie: `Sennheiser`)
 
 (I personally have my polybar configuration setup to show the output of `puslecycle -c` and when I click on it pulsecycle cycles to the next audio output.)
 
 
`pulsecycle -l`

This will show the current sink in long format. (ie: `USB Audio Analog Stereo`)
