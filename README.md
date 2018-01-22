# Rust Lisp

## Code Samples

``` clojure
(~>
  (range~ 1 1000)
  (select~ &(or (divides? 3 &1) (divides? 5 &1)))
  sum~
  (println "Solution: "))
```

## Problems for Performance Testing

* 14, runs in ~22s
* 23, runs in ~62s
