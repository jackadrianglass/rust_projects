# Ideas

* Generator has including and excluding letters or numbers
* Generator that generates sequence of numbers that add up to the number of notes in subdivision
    * A la Benny Greb with tuples and groupings
    * Include generation from [3, 4, 5, 6, 7]
    * Later, weight 3s, 5s and 7s higher
    * eg 16ths 1 bar can generate 3-5-7-1 adds up to 16

CLI
* Option for number of bars
* Option for subdivision. One of [4, 8, 16, 12, 24] for now
* Display using O--- notation and group by subdivision
    * eg in 8 it would be O- -- O- --

Ascii music notation?
8th notes
 ___
|   |
x   x

16th notes
 _____
|-|-|-|
x x x x

32nd notes
 ______
|=|=|=|=
xxxxxxxx

8th note triplets
  3
 ___ 
| | |
x x x

16th note triplets
  3
 ___
|-|-|
xxxxx


Simple beat
 ___     ___     ___     ___
|   |   |   |   |   |   |   |
H   H   H   H   H   H   H   H
|       S       |       S
B               B 
 _____   _____   _____   _____
|-|-|-| |-|-|-| |-|-|-| |-|-|-|
H H H H H H H H H H H H H H H H
|       S       |       S
B               B 
