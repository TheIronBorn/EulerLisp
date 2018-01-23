# EulerLisp

Progress:

![](https://projecteuler.net/profile/leonrische.png)

## Code Samples

``` clojure
(~>
  (range~ 1 1000)
  (select~ &(or (divides? 3 &1) (divides? 5 &1)))
  sum~
  (println "Solution: "))
```

## Notation for Project Euler Filenames

* `00.scm` Solution that runs in < 60s
* `00_T.scm` TODO, Solution that is gives the correct answer in < 60s,
  but I'm not sure if the math behind it is always valid
* `00_S.scm` SLOW, Solution that is correct but to slow
* `00_W.scm` WIP, Work-in-progress

## Problems for Performance Testing

* 14, runs in ~22s
* 23, runs in ~62s
