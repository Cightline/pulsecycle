# pulsecycle
A tool to switch your pulseaudio outputs on Linux. This is a Rust rewrite of [switch_stream](https://github.com/Cightline/switch_stream). 

<sup>(note: This was hastily written after getting tired of Python.)</sup>

### There are 3 main command line options right now. 

 ```pulsecycle -c```

 Cycle to the next available pulseaudio sink moving all streams along with it.
 
`pulsecycle -s`

 Show the current sink in short format. (ie: `Sennheiser`)
 
 (I personally have my polybar configuration setup to show the output of `puslecycle -c` and when I click on it pulsecycle cycles to the next audio output.)
 
 
`pulsecycle -l`

Show the current sink in long format. (ie: `USB Audio Analog Stereo`)
