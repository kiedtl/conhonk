# conhonk [![Build Status](https://travis-ci.org/lptstr/conhonk.svg?branch=master)](https://travis-ci.org/lptstr/conhonk)
Make your console beep according to the provided frequency and duration.

## Installation
- Try using [Scoop](https://scoop.sh). <br>
  - First, add the LPTSTR-Scoop bucket (if you haven't already):
    ```
    $ scoop bucket add lptstr https://github.com/lptstr/lptstr-scoop
    ```
  - Now install the application:
    ```
    $ scoop install conhonk
    ```
- If you aren't interested in using a package manager, just download `painter.exe` from the latest release in the latest [releases section](https://github.com/lptstr/painter/releases).

## Usage 
```
$ ./conhonk --frequency <freq> --duration <milliseconds>
```
| Option | Short | Default | Description |
|:------:|:-----:|:-------:|:-----------:|
|`--frequency`|`-f`|800|The frequency of the tone to be emitted|
|`--duration`|`-d`|200|The duration (in milleseconds) of the tone to be emitted|

E.g. To beep for 5 seconds on a frequency of 800, run
```
$ ./conhonk --frequency 800 --duration 5000
```
The above could be shortened to:
```
$ ./conhonk -d 5000
```
### Pranks
Add this code to somebody's PowerShell profile:
```powershell
while ($true) {
  ./conhonk -f 80000 -d 5000
}
```

## Inspiration
- none
